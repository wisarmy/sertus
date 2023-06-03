use dialoguer::{console::Style, theme::ColorfulTheme, Confirm, Input, Select};
use sconfig::Configurable;
use sertus::{
    checker::{process::ProcessChecker, script::ScriptChecker, Checker},
    config::Config,
    error::Result,
    flow::Flow,
    metrics::{Metrics, PushGateway, Server},
    task::Task,
};
pub fn default(force: bool) -> Result<()> {
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
    Ok(())
}

pub fn interact(mut force: bool) -> Result<()> {
    let theme = ColorfulTheme {
        values_style: Style::new().yellow().dim(),
        ..ColorfulTheme::default()
    };

    let mut config = Config::default();

    let metrics_item = Select::with_theme(&theme)
        .with_prompt("How to use metrics")
        .default(0)
        .item("Server")
        .item("Pushgateway")
        .interact()?;
    config.metrics = match metrics_item {
        0 => Metrics::Server(Server {
            addr: Input::with_theme(&theme)
                .with_prompt("addr")
                .default(Server::default().addr)
                .interact()?,
            ..Default::default()
        }),
        1 => Metrics::PushGateway(PushGateway {
            endpoint: Input::with_theme(&theme)
                .with_prompt("endpoint")
                .default(PushGateway::default().endpoint)
                .interact()?,
            ..Default::default()
        }),
        _ => unreachable!(),
    };
    let mut flow1 = Flow::new(
        Input::with_theme(&theme)
            .with_prompt("flow name")
            .default("flow 1".to_string())
            .interact()?,
    );
    let task_name = Input::with_theme(&theme)
        .with_prompt("task name")
        .default("task 1".to_string())
        .interact()?;
    let checker_item = Select::with_theme(&theme)
        .with_prompt("checker type")
        .default(0)
        .item("Process")
        .item("Script")
        .interact()?;
    let checker = match checker_item {
        0 => Checker::ProcessChecker(ProcessChecker {
            prefix: Input::with_theme(&theme)
                .with_prompt("prefix")
                .default("process prefix".to_string())
                .interact()?,
        }),
        1 => Checker::ScriptChecker(ScriptChecker {
            path: Input::with_theme(&theme)
                .with_prompt("path")
                .default("~/.sertus/scripts/script.sh".to_string())
                .interact()?,
            bin: Some(
                Input::with_theme(&theme)
                    .with_prompt("bin")
                    .default("bash".to_string())
                    .interact()?,
            ),
        }),
        _ => unreachable!(),
    };
    flow1.add_task(Task::new(task_name, checker));
    config.add_flow(flow1);
    if !force {
        force = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("If the config file already exists, would you like to overwrite it?")
            .interact()?;
    }
    config.init(force)?;
    Ok(())
}
