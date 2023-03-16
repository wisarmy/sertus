use clap::{Parser, Subcommand};
use config::Configurable;
use sertus::{
    checker::process::ProcessChecker,
    checker::Checker,
    config::{with_config, Config},
    error::Result,
    executor::Executor,
    flow::Flow,
    metrics::start_metrics_server,
    task::Task,
};
use tracing::{debug, error, info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Sertus program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    commnad: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Init {
        #[arg(short, long)]
        force: Option<bool>,
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
            let mut flow2 = Flow::new("flow 2");
            flow1
                .add_task(Task::new(
                    "task 1",
                    Checker::ProcessChecker(ProcessChecker::new("process prefix")),
                ))
                .add_task(Task::new(
                    "task 2",
                    Checker::ProcessChecker(ProcessChecker::new("process prefix")),
                ));
            flow2
                .add_task(Task::new(
                    "task 1",
                    Checker::ProcessChecker(ProcessChecker::new("process prefix")),
                ))
                .add_task(Task::new(
                    "task 2",
                    Checker::ProcessChecker(ProcessChecker::new("process prefix")),
                ));
            config.add_flow(flow1).add_flow(flow2);

            config.init(force.unwrap_or_default())?;
        }
        Command::Daemon => {
            info!("Initializing daemon");
            with_config(|c| async move {
                debug!("With config: {:#?}", c);

                let mut labels: Vec<(&str, String)> = vec![];
                tokio::spawn(start_metrics_server(c.metrics.addr, c.metrics.bucket));

                for flow in c.flows {
                    labels.push(("flow", flow.name.clone()));
                    // TODO flow timer
                    debug!("Starting Flow({} {})", flow.name, "-".repeat(30));
                    for task in flow.tasks {
                        labels.push(("task", task.name.clone()));
                        debug!("Running Task({}), {:?}", task.name, task.checker);
                        match task.checker.exec().await {
                            Ok(output) => {
                                if output {
                                    info!("Succeeded Task({})", task.name);
                                    metrics::increment_counter!(
                                        "sertus_flow_task_succeed",
                                        &labels
                                    );
                                } else {
                                    warn!("Failed Task({})", task.name);
                                    metrics::increment_counter!("sertus_flow_task_fail", &labels);
                                }
                            }
                            Err(e) => {
                                metrics::increment_counter!("sertus_flow_task_error", &labels);
                                error!("Error Task({}), {}", task.name, e);
                            }
                        }
                    }
                    metrics::increment_counter!("sertus_flow_ok", &labels);
                    debug!("Ended Flow({} {})", flow.name, "-".repeat(30));
                }
            })
            .await;
            std::future::pending::<()>().await;
        }
    }

    Ok(())
}
