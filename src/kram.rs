use std::{
    path::{Path, PathBuf},
    process::Command,
};

use which::which;

use crate::{CompressionFormat, Compressor, ContainerFormat, TextureType};

pub const SUPPORTED_COMPRESSION_FORMATS: [CompressionFormat; 13] = [
    CompressionFormat::Astc4x4,
    CompressionFormat::Astc5x5,
    CompressionFormat::Astc6x6,
    CompressionFormat::Astc8x8,
    CompressionFormat::Bc1,
    CompressionFormat::Bc3,
    CompressionFormat::Bc4,
    CompressionFormat::Bc5,
    CompressionFormat::Bc7,
    CompressionFormat::Etc2r,
    CompressionFormat::Etc2rg,
    CompressionFormat::Etc2rgb,
    CompressionFormat::Etc2rgba,
];

pub struct Kram {
    cli_path: PathBuf,
}

impl Kram {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            cli_path: which("kram").map_err(|e| {
                format!(
                    "Failed to find the kram CLI tool. Make sure it is in your PATH. {:?}",
                    e
                )
            })?,
        })
    }
}

impl Compressor for Kram {
    fn compress<D: AsRef<Path>>(
        &self,
        working_dir: D,
        src_path: D,
        dst_path: D,
        texture_type: TextureType,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String> {
        if !SUPPORTED_COMPRESSION_FORMATS.contains(&compression_format)
            || !matches!(container_format, ContainerFormat::Ktx2)
        {
            return Err(format!(
                "Unsupported format {:?} {:?} - must be one of {:?}, and {}",
                compression_format,
                container_format,
                SUPPORTED_COMPRESSION_FORMATS,
                ContainerFormat::Ktx2,
            ));
        }
        let mut command = Command::new(&self.cli_path);
        command.current_dir(working_dir.as_ref());
        command.args([
            "encode",
            "-input",
            src_path.as_ref().to_str().unwrap(),
            "-output",
            dst_path.as_ref().to_str().unwrap(),
            "-mipmin",
            "1",
            "-zstd",
            "0",
        ]);
        match compression_format {
            CompressionFormat::Astc4x4 => {
                command.args(["-format", "astc4x4", "-encoder", "astcenc"]);
            }
            CompressionFormat::Astc5x5 => {
                command.args(["-format", "astc5x5", "-encoder", "astcenc"]);
            }
            CompressionFormat::Astc6x6 => {
                command.args(["-format", "astc6x6", "-encoder", "astcenc"]);
            }
            CompressionFormat::Astc8x8 => {
                command.args(["-format", "astc8x8", "-encoder", "astcenc"]);
            }
            CompressionFormat::Bc1 => {
                command.args(["-format", "bc1", "-encoder", "bcenc"]);
            }
            CompressionFormat::Bc3 => {
                command.args(["-format", "bc3", "-encoder", "bcenc"]);
            }
            CompressionFormat::Bc4 => {
                command.args(["-format", "bc4", "-encoder", "bcenc"]);
            }
            CompressionFormat::Bc5 => {
                command.args(["-format", "bc5", "-encoder", "bcenc"]);
            }
            CompressionFormat::Bc7 => {
                command.args(["-format", "bc7", "-encoder", "bcenc"]);
            }
            CompressionFormat::Etc2r => {
                command.args(["-format", "etc2r", "-encoder", "etcenc"]);
            }
            CompressionFormat::Etc2rg => {
                command.args(["-format", "etc2rg", "-encoder", "etcenc"]);
            }
            CompressionFormat::Etc2rgb => {
                command.args(["-format", "etc2rgb", "-encoder", "etcenc"]);
            }
            CompressionFormat::Etc2rgba => {
                command.args(["-format", "etc2rgba", "-encoder", "etcenc"]);
            }
            _ => {
                return Err(format!(
                    "Unsupported format {:?} {:?} - must be one of {:?} and {}",
                    compression_format,
                    container_format,
                    SUPPORTED_COMPRESSION_FORMATS,
                    ContainerFormat::Ktx2,
                ));
            }
        }
        match texture_type {
            TextureType::Srgb => {
                command.arg("-srgb");
            }
            TextureType::Linear => {}
            TextureType::NormalMap => {
                command.arg("-normal");
                match compression_format {
                    CompressionFormat::Astc4x4
                    | CompressionFormat::Astc5x5
                    | CompressionFormat::Astc6x6
                    | CompressionFormat::Astc8x8 => {
                        command.args(["-swizzle", "rrrg"]);
                    }
                    _ => {}
                }
            }
        }
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
