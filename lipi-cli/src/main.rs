use clap::{Parser, Subcommand};

mod commands;
mod config;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Log { message: String },
    Generate,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run(),
        Commands::Log { message } => commands::log::run(message).await,
        Commands::Generate => commands::generate::run().await,
    }
}