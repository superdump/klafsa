use clap::{Parser, Subcommand};
use klafsa::{Backend, CompressionFormat, ContainerFormat, Gltf};
use tracing::{error, subscriber};
use tracing_subscriber::{fmt, prelude::*, EnvFilter, Registry};

/// Texture compression tool for converting JPEG/PNG to various compressed formats
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
    /// Which tool to use for compression
    #[clap(short, long, arg_enum, default_value_t = Backend::Toktx)]
    backend: Backend,
    /// Which codec to use for compression
    #[clap(long, arg_enum, default_value_t = CompressionFormat::Uastc)]
    codec: CompressionFormat,
    /// Which container format to use
    #[clap(long, arg_enum, default_value_t = ContainerFormat::Ktx2)]
    container: ContainerFormat,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Converts all JPEG/PNG textures referred to by a JSON-format glTF
    Gltf(Gltf),
}

fn main() {
    init_logging();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Gltf(gltf) => {
            if let Err(e) = gltf.process(cli.backend, cli.codec, cli.container) {
                error!("Failed to process '{}' - {}", gltf.file_path, e);
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
