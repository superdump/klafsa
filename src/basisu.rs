use std::{
    path::{Path, PathBuf},
    process::Command,
};

use which::which;

use crate::{CompressionFormat, Compressor, ContainerFormat, TextureType};

pub const SUPPORTED_COMPRESSION_FORMATS: [CompressionFormat; 2] =
    [CompressionFormat::Etc1s, CompressionFormat::Uastc];

pub const SUPPORTED_CONTAINER_FORMATS: [ContainerFormat; 2] =
    [ContainerFormat::Basis, ContainerFormat::Ktx2];

pub struct Basisu {
    cli_path: PathBuf,
}

impl Basisu {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            cli_path: which("basisu").map_err(|e| {
                format!(
                    "Failed to find the basisu CLI tool. Make sure it is in your PATH. {:?}",
                    e
                )
            })?,
        })
    }
}

impl Compressor for Basisu {
    fn compress(
        &self,
        working_dir: &Path,
        src_path: &Path,
        dst_path: &Path,
        texture_type: TextureType,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String> {
        if !SUPPORTED_COMPRESSION_FORMATS.contains(&compression_format)
            || !SUPPORTED_CONTAINER_FORMATS.contains(&container_format)
        {
            return Err(format!(
                "Unsupported format {:?} {:?} - must be one of {:?} and {}",
                compression_format,
                container_format,
                SUPPORTED_COMPRESSION_FORMATS,
                ContainerFormat::Ktx2,
            ));
        }
        let mut command = Command::new(&self.cli_path);
        command.current_dir(working_dir);
        command.args([
            src_path.to_str().unwrap(),
            "-output_file",
            dst_path.to_str().unwrap(),
            "-mipmap",
            "-mip_fast",
        ]);
        if matches!(compression_format, CompressionFormat::Uastc) {
            command.arg("-uastc");
        }
        if matches!(container_format, ContainerFormat::Ktx2) {
            command.arg("-ktx2");
        }
        match texture_type {
            TextureType::Srgb => command.arg("-mip_srgb"),
            TextureType::Linear => command.args(["-linear", "-mip_linear"]),
            TextureType::NormalMap => command.args(["-normal_map", "-linear", "-mip_linear"]),
        };
        match command.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!(
                        "Failed to execute command:\n{}",
                        std::str::from_utf8(&output.stderr).unwrap()
                    ))
                }
            }
            Err(e) => Err(format!("Failed to execute command: {:?}", e)),
        }
    }
}
