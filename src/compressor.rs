use std::path::Path;

use clap::ArgEnum;

use crate::Backend;

#[derive(Clone, Copy, Debug)]
pub enum TextureType {
    Srgb,
    Linear,
    NormalMap,
}

#[derive(
    Clone, Copy, Debug, ArgEnum, strum::Display, strum::EnumIter, strum::EnumString, PartialEq, Eq,
)]
#[strum(serialize_all = "lowercase")]
pub enum CompressionFormat {
    Astc,
    Astc4x4,
    Astc5x5,
    Astc6x6,
    Astc8x8,
    Bc1,
    Bc3,
    Bc4,
    Bc5,
    Bc7,
    Etc1s,
    Etc2r,
    Etc2rg,
    Etc2rgb,
    Etc2rgba,
    Uastc,
}

impl CompressionFormat {
    pub fn backend(&self) -> Option<Backend> {
        Some(match *self {
            CompressionFormat::Astc4x4 => Backend::Kram,
            CompressionFormat::Bc1 => Backend::Kram,
            CompressionFormat::Bc3 => Backend::Kram,
            CompressionFormat::Bc4 => Backend::Kram,
            CompressionFormat::Bc5 => Backend::Kram,
            CompressionFormat::Bc7 => Backend::Kram,
            CompressionFormat::Etc1s => Backend::Basisu,
            CompressionFormat::Etc2r => Backend::Kram,
            CompressionFormat::Etc2rg => Backend::Kram,
            CompressionFormat::Etc2rgb => Backend::Kram,
            CompressionFormat::Etc2rgba => Backend::Kram,
            CompressionFormat::Uastc => Backend::Basisu,
            _ => return None,
        })
    }

    pub fn container(&self) -> ContainerFormat {
        if matches!(*self, CompressionFormat::Etc1s) {
            ContainerFormat::Basis
        } else {
            ContainerFormat::Ktx2
        }
    }
}

#[derive(Clone, Copy, Debug, ArgEnum, strum::Display, strum::EnumString, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum ContainerFormat {
    Basis,
    Ktx2,
}

pub trait Compressor {
    fn compress(
        &self,
        working_dir: &Path,
        src_path: &Path,
        dst_path: &Path,
        texture_type: TextureType,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String>;
}
