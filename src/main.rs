use clap::Parser;

use lamarck::{generate_captions, Cli, Commands};

use miette::{IntoDiagnostic, Result};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // You can check for the existence of subcommands,
    // and if found use their matches just as you
    // would the top level cmd
    let result = match &cli.command {
        Commands::Caption(options) => {
            generate_captions(options)
        }
    }
    .await;
    dbg!(&result);
    result.into_diagnostic()
}
