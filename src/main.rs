pub mod api;
pub mod app;
mod auth;
pub mod db;
pub mod pings;
pub mod redis;
mod server;
mod worker;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "App")]
#[command(about = "An application with async server and worker subcommands", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the async server
    Server,
    /// Start the async worker
    Worker,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    dotenv::dotenv().ok();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Server => server::main().await,
        Commands::Worker => worker::main().await,
    }
}
