use std::path::Path;

#[derive(Debug)]
pub enum TextureType {
    Srgb,
    Linear,
    NormalMap,
}

#[derive(Clone, Copy, Debug, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum CompressionFormat {
    Bc7,
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
