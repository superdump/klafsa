use std::{
    path::{Path, PathBuf},
    process::Command,
};

use which::which;

use crate::{CompressionFormat, Compressor, ContainerFormat, TextureType};

pub struct BasisU {
    cli_path: PathBuf,
}

impl BasisU {
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

impl Compressor for BasisU {
    fn compress<D: AsRef<Path>>(
        &self,
        working_dir: D,
        src_path: D,
        dst_path: D,
        texture_type: TextureType,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String> {
        if !matches!(compression_format, CompressionFormat::Uastc)
            || !matches!(container_format, ContainerFormat::Ktx2)
        {
            return Err(format!(
                "Unsupported format {:?} {:?} - must be UASTC and KTX2",
                compression_format, container_format
            ));
        }
        let mut command = Command::new(&self.cli_path);
        command.current_dir(working_dir.as_ref());
        command.args([
            src_path.as_ref().to_str().unwrap(),
            "-output_file",
            dst_path.as_ref().to_str().unwrap(),
            "-mipmap",
            "-mip_fast",
            "-uastc",
            "-ktx2",
        ]);
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
            compression_format.unwrap_or(CompressionFormat::Uastc),
            container_format.unwrap_or(ContainerFormat::Ktx2),
        )
    }
}
