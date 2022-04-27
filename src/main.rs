use clap::{ArgEnum, Args, Parser, Subcommand};
use klafsa::{process_gltf, CompressionFormat, ContainerFormat};
use tracing::{error, info, subscriber};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

/// Texture compression tool for converting JPEG/PNG to various compressed formats
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// Which tool to use for compression
    #[clap(short, long, arg_enum, default_value_t = Backend::ToKtx)]
    backend: Backend,
    /// Which codec to use for compression
    #[clap(long, arg_enum)]
    codec: Option<CompressionFormat>,
    /// Which container format to use
    #[clap(long, arg_enum)]
    container: Option<ContainerFormat>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Converts all JPEG/PNG textures referred to by a JSON-format glTF
    Gltf(Gltf),
}

#[derive(Args, Debug)]
struct Gltf {
    file_path: String,
}

#[derive(Clone, Debug, ArgEnum, strum::Display, strum::EnumString)]
#[strum(serialize_all = "lowercase")]
enum Backend {
    BasisU,
    Kram,
    ToKtx,
}

fn main() {
    init_logging();

    let cli = Cli::parse();

    let compressor = match cli.backend {
        Backend::BasisU => klafsa::Backend::BasisU(match klafsa::BasisU::new() {
            Ok(basisu) => basisu,
            Err(e) => {
                error!("Failed to create compressor: {}", e);
                std::process::exit(1);
            }
        }),
        Backend::Kram => klafsa::Backend::Kram(match klafsa::Kram::new() {
            Ok(kram) => kram,
            Err(e) => {
                error!("Failed to create compressor: {}", e);
                std::process::exit(1);
            }
        }),
        Backend::ToKtx => klafsa::Backend::ToKtx(match klafsa::ToKtx::new() {
            Ok(kram) => kram,
            Err(e) => {
                error!("Failed to create compressor: {}", e);
                std::process::exit(1);
            }
        }),
    };

    match &cli.command {
        Commands::Gltf(gltf) => {
            if !gltf.file_path.to_lowercase().ends_with(".gltf") {
                error!("File must be a JSON-format glTF file with a .gtlf file extension");
                std::process::exit(1);
            }

            info!("Processing {}", gltf.file_path);

            if let Err(e) = process_gltf(&gltf.file_path, &compressor, cli.codec, cli.container) {
                error!("Failed to process glTF file: {} - {}", gltf.file_path, e);
                std::process::exit(1);
            }
        }
    }

    std::process::exit(0)
}

fn init_logging() {
    let subscriber = Registry::default().with(fmt::layer()).with(
        EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap(),
    );
    subscriber::set_global_default(subscriber).expect("Failed to set up tracing subscriber");
}
