use clap::{Parser, Subcommand};
use sconfig::Configurable;
use sertus::{
    checker::Checker,
    checker::{process::ProcessChecker, script::ScriptChecker},
    config::{with_config, Config},
    error::Result,
    flow::Flow,
    metrics::{setup_pushgateway, start_metrics_server, Metrics},
    pkg::version,
    task::Task,
};
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Sertus program
#[derive(Parser, Debug)]
#[command(author, version=version::default(), about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commnad: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[clap(short, long)]
        force: bool,
    },
    Daemon,
}
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    match cli.commnad {
        Command::Init { force } => {
            // init
            let mut config = Config::default();
            let mut flow1 = Flow::new("flow 1");
            flow1
                .add_task(Task::new(
                    "check process",
                    Checker::ProcessChecker(ProcessChecker::new("process prefix")),
                ))
                .add_task(Task::new(
                    "check script",
                    Checker::ScriptChecker(ScriptChecker::new("~/.sertus/scripts/script.sh")),
                ));
            config.add_flow(flow1);

            config.init(force)?;
        }
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
    }

    Ok(())
}
