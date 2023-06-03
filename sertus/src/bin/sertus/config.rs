use dialoguer::Editor;
use sconfig::Configurable;
use sertus::config::{with_config, Config};
use tracing::error;

pub async fn editor() {
    with_config(|c| async move {
        if let Some(rv) = Editor::new().edit(c.to_string().as_str()).unwrap() {
            let new_config = rv
                .parse::<Config>()
                .inspect_err(|e| error!("config parse err: {}", e))
                .ok();
            if let Some(new_config) = new_config {
                if let Err(e) = new_config.write() {
                    error!("config save err: {}", e);
                } else {
                    println!("Config has been updated:");
                    println!("{}\n", rv);
                }
            }
        } else {
            println!("Abort!");
        }
    })
    .await;
}
