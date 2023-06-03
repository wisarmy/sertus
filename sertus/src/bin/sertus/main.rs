#![feature(result_option_inspect)]
use clap::{Parser, Subcommand};
use sertus::{
    config::with_config,
    error::Result,
    metrics::{setup_pushgateway, start_metrics_server, Metrics},
    pkg::version,
};
use tracing::{debug, info};
use tracing_subscriber::{fmt::time::LocalTime, layer::SubscriberExt, util::SubscriberInitExt};

pub mod config;
pub mod init;

/// Sertus program
#[derive(Parser, Debug)]
#[command(author, version=version::default(), about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commnad: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize config
    Init {
        /// interactively create config
        #[clap(short, long)]
        interact: bool,
        /// force overwrite of existing config
        #[clap(short, long)]
        force: bool,
    },
    /// Run daemon
    Daemon,
    /// Config subcommands
    #[clap(subcommand)]
    Config(ConfigCommand),
}

#[derive(Subcommand, Debug)]
enum ConfigCommand {
    /// Edit config
    Edit,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(LocalTime::rfc_3339()))
        .init();

    let cli = Cli::parse();
    match cli.commnad {
        Command::Init { interact, force } => match interact {
            true => init::interact(force)?,
            false => init::default(force)?,
        },
        Command::Daemon => {
            info!("Initializing daemon");
            with_config(|c| async move {
                debug!("With config: {:#?}", c);
                match c.metrics {
                    Metrics::Server(s) => {
                        tokio::spawn(start_metrics_server(s));
                    }
                    Metrics::PushGateway(p) => {
                        tokio::spawn(setup_pushgateway(p));
                    }
                }

                for flow in c.flows.into_iter() {
                    tokio::spawn(flow.run());
                }
            })
            .await;
            std::future::pending::<()>().await;
        }
        Command::Config(config_command) => match config_command {
            ConfigCommand::Edit => {
                config::editor().await;
            }
        },
    }

    Ok(())
}
