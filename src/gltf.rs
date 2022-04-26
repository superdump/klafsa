use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use gltf::json::{image::MimeType, Root};
use indicatif::{ProgressBar, ProgressStyle};
use tracing::{error, info, warn};

use crate::{image::ImageFormat, CompressionFormat, Compressor, ContainerFormat, TextureType};

pub fn process_gltf<C: Compressor>(
    src_path: &str,
    compressor: &C,
    compression_format: Option<CompressionFormat>,
    container_format: Option<ContainerFormat>,
) -> Result<(), String> {
    let gltf = gltf::Gltf::open(src_path).expect("Failed to open file");
    let working_dir = Path::new(src_path).parent().map_or_else(
        || std::env::current_dir().expect("Failed to get parent directory of the glTF file"),
        |p| p.into(),
    );
    let (compression_format, container_format) =
        compressor.get_formats(compression_format, container_format);

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

    let mut gltf_root = read_gltf_to_json(src_path);

    let progress_bar = ProgressBar::new(gltf.textures().len() as u64).with_style(
        ProgressStyle::default_bar()
            .template("{pos}/{len} [{elapsed_precise}]/[{duration_precise}] {wide_bar} {msg}"),
    );
    progress_bar.enable_steady_tick(1000);

    for texture in gltf.textures() {
        match texture.source().source() {
            gltf::image::Source::View { mime_type, .. } => {
                warn!("Cannot process texture views. (Mime-type: {})", mime_type);
                continue;
            }
            gltf::image::Source::Uri { uri, mime_type } => {
                if ImageFormat::from_mime_or_extension(mime_type, Some(uri)).is_none() {
                    warn!("Unsupported image format");
                    continue;
                }
                let dst_path = format!("{}.ktx2", uri.rsplit_once('.').unwrap().0);
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
                if let Err(e) = compressor.compress(
                    working_dir.to_str().unwrap(),
                    uri,
                    dst_path.as_str(),
                    texture_type,
                    compression_format,
                    container_format,
                ) {
                    return Err(format!("{} - {}", uri, e));
                }
                gltf_root.images[texture.source().index()].mime_type =
                    Some(MimeType(String::from("image/ktx2")));
                gltf_root.images[texture.source().index()].uri = Some(format!(
                    "{}.ktx2",
                    gltf_root.images[texture.source().index()]
                        .uri
                        .as_ref()
                        .unwrap()
                        .rsplit_once('.')
                        .unwrap()
                        .0
                ));
            }
        }
        progress_bar.inc(1);
    }
    progress_bar.finish();

    let dst_path = src_path
        .rsplit_once('.')
        .map(|(path, extension)| {
            format!(
                "{}_{}_{}.{}",
                path, compression_format, container_format, extension
            )
        })
        .expect("Failed to create compressed glTF filename");
    write_json_to_gltf(gltf_root, &dst_path);

    Ok(())
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
