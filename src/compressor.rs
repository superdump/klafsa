use std::path::Path;

#[derive(Debug)]
pub enum TextureType {
    Srgb,
    Linear,
    NormalMap,
}

#[derive(Clone, Copy, Debug, strum::Display, strum::EnumString, PartialEq, Eq, Hash)]
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
    Etc2R,
    Etc2Rg,
    Etc2Rgb,
    Etc2Rgba,
    Uastc,
}

#[derive(Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum ContainerFormat {
    Ktx2,
}

pub trait Compressor {
    fn compress<P: AsRef<Path>>(
        &self,
        working_dir: P,
        src_path: P,
        dst_path: P,
        texture_type: TextureType,
        compression_format: CompressionFormat,
        container_format: ContainerFormat,
    ) -> Result<(), String>;

    fn get_formats(
        &self,
        compression_format: Option<CompressionFormat>,
        container_format: Option<ContainerFormat>,
    ) -> (CompressionFormat, ContainerFormat);
}
