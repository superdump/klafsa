mod basisu;
mod compressor;
mod gltf;
mod image;
mod kram;
mod toktx;

use std::path::Path;

pub use crate::gltf::*;
pub use basisu::*;
pub use compressor::*;
pub use kram::*;
pub use toktx::*;

pub enum Backend {
    BasisU(BasisU),
    Kram(Kram),
    ToKtx(ToKtx),
}

impl Compressor for Backend {
    fn compress<P: AsRef<Path>>(
        &self,
        working_dir: P,
        src_path: P,
        dst_path: P,
        texture_type: TextureType,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String> {
        match self {
            Backend::BasisU(basisu) => basisu.compress(
                working_dir,
                src_path,
                dst_path,
                texture_type,
                compression_format,
                container_format,
            ),
            Backend::Kram(kram) => kram.compress(
                working_dir,
                src_path,
                dst_path,
                texture_type,
                compression_format,
                container_format,
            ),
            Backend::ToKtx(toktx) => toktx.compress(
                working_dir,
                src_path,
                dst_path,
                texture_type,
                compression_format,
                container_format,
            ),
        }
    }

    fn get_formats(
        &self,
        compression_format: Option<CompressionFormat>,
        container_format: Option<ContainerFormat>,
    ) -> (CompressionFormat, ContainerFormat) {
        match self {
            Backend::BasisU(basisu) => basisu.get_formats(compression_format, container_format),
            Backend::Kram(kram) => kram.get_formats(compression_format, container_format),
            Backend::ToKtx(toktx) => toktx.get_formats(compression_format, container_format),
        }
    }
}
