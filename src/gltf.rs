use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use clap::Args;
use gltf::json::{image::MimeType, Root};
use indicatif::{ProgressBar, ProgressStyle};
use strum::IntoEnumIterator;
use tracing::{error, info, warn};

use crate::{
    image::ImageFormat, Backend, Basisu, CompressionFormat, Compressor, ContainerFormat, Kram,
    TextureType, Toktx,
};

#[derive(Args, Debug)]
pub struct Gltf {
    /// Path to the JSON-format .gltf file
    pub file_path: String,
    /// Compress to all formats
    #[clap(long)]
    compress_to_all: bool,
}

impl Gltf {
    pub fn process(
        &self,
        backend: Backend,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String> {
        if !self.file_path.to_lowercase().ends_with(".gltf") {
            error!("File must be a JSON-format glTF file with a .gtlf file extension");
            std::process::exit(1);
        }

        info!("Processing {}", self.file_path);

        let gltf = gltf::Gltf::open(&self.file_path).expect("Failed to open file");
        let working_dir = Path::new(&self.file_path).parent().map_or_else(
            || std::env::current_dir().expect("Failed to get parent directory of the glTF file"),
            |p| p.into(),
        );

        let mut normal_map_textures = HashSet::new();
        let mut linear_textures = HashSet::new();
        for material in gltf.materials() {
            if let Some(texture) = material.normal_texture() {
                normal_map_textures.insert(texture.texture().index());
            }
            if let Some(texture) = material.occlusion_texture() {
                linear_textures.insert(texture.texture().index());
            }
            if let Some(texture) = material
                .pbr_metallic_roughness()
                .metallic_roughness_texture()
            {
                linear_textures.insert(texture.texture().index());
            }
        }

        let formats = self.get_formats(compression_format);
        let compressors = self.get_compressors(backend);

        let gltf_root = read_gltf_to_json(&self.file_path);
        let mut gltf_roots = vec![gltf_root; formats.len()];

        let progress_bar = ProgressBar::new((formats.len() * gltf.textures().len()) as u64)
            .with_style(
                ProgressStyle::default_bar().template(
                    "{pos}/{len} [{elapsed_precise}]/[{duration_precise}] {wide_bar} {msg}",
                ),
            );
        progress_bar.enable_steady_tick(1000);

        for texture in gltf.textures() {
            match texture.source().source() {
                gltf::image::Source::View { mime_type, .. } => {
                    warn!("Cannot process texture views. (Mime-type: {})", mime_type);
                    progress_bar.inc(formats.len() as u64);
                    continue;
                }
                gltf::image::Source::Uri { uri, mime_type } => {
                    if ImageFormat::from_mime_or_extension(mime_type, Some(uri)).is_none() {
                        warn!("Unsupported image format");
                        progress_bar.inc(formats.len() as u64);
                        continue;
                    }
                    let texture_type = if linear_textures.contains(&texture.index()) {
                        TextureType::Linear
                    } else if normal_map_textures.contains(&texture.index()) {
                        TextureType::NormalMap
                    } else {
                        TextureType::Srgb
                    };
                    progress_bar.set_message(
                        Path::new(uri)
                            .file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap(),
                    );
                    for (format, gltf_root) in formats.iter().zip(gltf_roots.iter_mut()) {
                        let src_path = Path::new(uri);
                        let container = self.get_container(*format, container_format);
                        let dst_path = src_path
                            .parent()
                            .unwrap()
                            .join(format!("{}_{}", format, container))
                            .join(format!(
                                "{}_{}.{}",
                                src_path.file_stem().unwrap().to_str().unwrap(),
                                format,
                                container
                            ));
                        if let Err(e) =
                            std::fs::create_dir_all(working_dir.join(dst_path.parent().unwrap()))
                        {
                            error!(
                                "Failed to recursively create directory: {} - {}",
                                dst_path.parent().unwrap().display(),
                                e
                            );
                            progress_bar.inc(1);
                            continue;
                        }
                        if let Err(e) = compressors[&format.backend().unwrap()].compress(
                            &working_dir,
                            src_path,
                            &dst_path,
                            texture_type,
                            *format,
                            container,
                        ) {
                            error!("{} -> {} - {}", uri, dst_path.display(), e);
                            progress_bar.inc(1);
                            continue;
                        }
                        gltf_root.images[texture.source().index()].mime_type = match container {
                            // NOTE: There is no valid official mime type for .basis files
                            ContainerFormat::Basis => None,
                            ContainerFormat::Ktx2 => Some(MimeType(String::from("image/ktx2"))),
                        };
                        gltf_root.images[texture.source().index()].uri =
                            Some(dst_path.display().to_string());
                    }
                }
            }
            progress_bar.inc(1);
        }
        progress_bar.finish();

        for (format, gltf_root) in formats.iter().zip(gltf_roots.into_iter()) {
            let dst_path = self
                .file_path
                .rsplit_once('.')
                .map(|(path, extension)| {
                    format!(
                        "{}_{}_{}.{}",
                        path,
                        format,
                        if self.compress_to_all {
                            format.container()
                        } else {
                            container_format
                        },
                        extension
                    )
                })
                .expect("Failed to create compressed glTF filename");
            write_json_to_gltf(gltf_root, &dst_path);
        }

        Ok(())
    }

    fn get_formats(&self, compression_format: CompressionFormat) -> Vec<CompressionFormat> {
        if self.compress_to_all {
            CompressionFormat::iter()
                .filter(|f| f.backend().is_some())
                .collect::<Vec<_>>()
        } else {
            vec![compression_format]
        }
    }

    fn get_compressors(&self, backend: Backend) -> HashMap<Backend, Box<dyn Compressor>> {
        let mut compressors: HashMap<Backend, Box<dyn Compressor>> = HashMap::new();
        if self.compress_to_all {
            compressors.insert(
                Backend::Basisu,
                Box::new(Basisu::new().expect("Failed to create basisu compressor")),
            );
            compressors.insert(
                Backend::Kram,
                Box::new(Kram::new().expect("Failed to create kram compressor")),
            );
            compressors.insert(
                Backend::Toktx,
                Box::new(Toktx::new().expect("Failed to create toktx compressor")),
            );
        } else {
            match backend {
                Backend::Basisu => {
                    compressors.insert(
                        Backend::Basisu,
                        Box::new(Basisu::new().expect("Failed to create basisu compressor")),
                    );
                }
                Backend::Kram => {
                    compressors.insert(
                        Backend::Kram,
                        Box::new(Kram::new().expect("Failed to create kram compressor")),
                    );
                }
                Backend::Toktx => {
                    compressors.insert(
                        Backend::Toktx,
                        Box::new(Toktx::new().expect("Failed to create toktx compressor")),
                    );
                }
            }
        }
        compressors
    }

    fn get_container(
        &self,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> ContainerFormat {
        if self.compress_to_all {
            compression_format.container()
        } else {
            container_format
        }
    }
}

fn read_gltf_to_json<P: AsRef<Path>>(src_path: P) -> Root {
    let file = File::open(src_path).expect("Failed to open glTF JSON file");
    let reader = BufReader::new(file);
    Root::from_reader(reader).expect("Failed to parse glTF JSON file")
}

fn write_json_to_gltf<P: AsRef<Path> + Copy>(root: Root, dst_path: P) {
    let file = File::create(dst_path).expect("Failed to open glTF JSON file");
    let writer = BufWriter::new(file);
    root.to_writer_pretty(writer)
        .map_err(|e| error!("{:?}", e))
        .expect("Failed to write glTF JSON file");
    info!("Wrote new glTF file at: {:?}", dst_path.as_ref());
}
