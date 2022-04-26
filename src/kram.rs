use std::{
    path::{Path, PathBuf},
    process::Command,
};

use which::which;

use crate::{CompressionFormat, Compressor, ContainerFormat, TextureType};

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
        if !matches!(compression_format, CompressionFormat::Bc7)
            || !matches!(container_format, ContainerFormat::Ktx2)
        {
            return Err(format!(
                "Unsupported format {:?} {:?} - must be BC7 and KTX2",
                compression_format, container_format
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
            "-format",
            "bc7",
            "-encoder",
            "bcenc",
            "-zstd",
            "0",
        ]);
        match texture_type {
            TextureType::Srgb => command.arg("-srgb"),
            TextureType::Linear => &mut command,
            TextureType::NormalMap => command.arg("-normal"),
        };
        match command.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(format!("Failed to execute command: {:?}", output))
                }
            }
            Err(e) => Err(format!("Failed to execute command: {:?}", e)),
        }
    }

    fn get_formats(
        &self,
        compression_format: Option<CompressionFormat>,
        container_format: Option<ContainerFormat>,
    ) -> (CompressionFormat, ContainerFormat) {
        (
            compression_format.unwrap_or(CompressionFormat::Bc7),
            container_format.unwrap_or(ContainerFormat::Ktx2),
        )
    }
}
