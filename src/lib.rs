mod basisu;
mod compressor;
mod gltf;
mod image;
mod kram;
mod toktx;

pub use crate::gltf::*;
pub use basisu::*;
use clap::ArgEnum;
pub use compressor::*;
pub use kram::*;
pub use toktx::*;

#[derive(Clone, Debug, ArgEnum, strum::Display, strum::EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Backend {
    Basisu,
    Kram,
    Toktx,
}
