use std::{
    path::{Path, PathBuf},
    process::Command,
};

use which::which;

use crate::{CompressionFormat, Compressor, ContainerFormat, TextureType};

pub const SUPPORTED_COMPRESSION_FORMATS: [CompressionFormat; 3] = [
    CompressionFormat::Astc,
    CompressionFormat::Etc1s,
    CompressionFormat::Uastc,
];

pub struct Toktx {
    cli_path: PathBuf,
}

impl Toktx {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            cli_path: which("toktx").map_err(|e| {
                format!(
                    "Failed to find the toktx CLI tool. Make sure it is in your PATH. {:?}",
                    e
                )
            })?,
        })
    }
}

impl Compressor for Toktx {
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
                "Unsupported format {:?} {:?} - must be one of {:?} and {}",
                compression_format,
                container_format,
                SUPPORTED_COMPRESSION_FORMATS,
                ContainerFormat::Ktx2,
            ));
        }
        let mut command = Command::new(&self.cli_path);
        command.current_dir(working_dir.as_ref());
        match texture_type {
            TextureType::Srgb => command.args(["--assign_oetf", "srgb"]),
            TextureType::Linear => command.args(["--assign_oetf", "linear"]),
            TextureType::NormalMap => command.args(["--normal_mode"]),
        };
        command.args([
            "--2d",
            "--genmipmap",
            "--encode",
            compression_format.to_string().as_str(),
            "--t2",
        ]);
        match compression_format {
            CompressionFormat::Etc1s => {}
            _ => {
                command.args(["--zcmp", "18"]);
            }
        }
        command.args([
            dst_path.as_ref().to_str().unwrap(),
            src_path.as_ref().to_str().unwrap(),
        ]);
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
