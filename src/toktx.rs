use std::{
    path::{Path, PathBuf},
    process::Command,
};

use which::which;

use crate::{CompressionFormat, Compressor, ContainerFormat, TextureType};

pub struct ToKtx {
    cli_path: PathBuf,
}

impl ToKtx {
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

impl Compressor for ToKtx {
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
        match texture_type {
            TextureType::Srgb => command.args(["--assign_oetf", "srgb"]),
            TextureType::Linear => command.args(["--assign_oetf", "linear"]),
            TextureType::NormalMap => command.args(["--normal_mode"]),
        };
        command.args([
            "--2d",
            "--genmipmap",
            "--encode",
            "uastc",
            "--t2",
            "--zcmp 18",
            dst_path.as_ref().to_str().unwrap(),
            src_path.as_ref().to_str().unwrap(),
        ]);
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
