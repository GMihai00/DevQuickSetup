use super::common::{expand_string_deserializer, ActionFn, InstallActionType};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::env;
use std::error::Error;
use std::process::Command;

use async_trait::async_trait;
use log::{debug, warn};

#[derive(Deserialize, Serialize)]
struct ExecCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    install_run: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    #[serde(default = "default_uninstall_run")]
    uninstall_run: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    #[serde(default = "default_update_run")]
    update_run: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    #[serde(default = "default_dir")]
    dir: String,
}

fn default_uninstall_run() -> String {
    return String::new();
}

fn default_update_run() -> String {
    return String::new();
}

fn default_dir() -> String {
    if let Ok(current_dir) = env::current_dir() {
        return current_dir.to_string_lossy().to_string();
    } else {
        warn!("Failed to get current directory");
        return String::new();
    }
}

impl ExecCommand {
    pub fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exec: &String;
        match action {
            InstallActionType::INSTALL => {
                exec = &self.install_run;
            }
            InstallActionType::UNINSTALL => {
                exec = &self.uninstall_run;
            }
            InstallActionType::UPDATE => {
                exec = &self.update_run;
            }
        }

        debug!("Executing command: \"{}\"", exec);

        if exec.len() == 0 {
            return Ok(true);
        }

        match shell_words::split(&exec).map(|parsed| {
            parsed
                .split_first()
                .map(|(exec, args)| (exec.to_string(), args.to_vec()))
        }) {
            Ok(Some((exec, args))) => {
                let status = Command::new(&exec)
                    .args(args)
                    .current_dir(&self.dir)
                    .spawn()
                    .expect("Failed to execute process")
                    .wait()
                    .expect("Failed to wait for process");

                return Ok(status.success());
            }
            Err(err) => {
                return Err(
                    format!("Failed to parse command line: \"{}\" err: {}", exec, err).into(),
                );
            }
            Ok(None) => {
                return Err(format!("Failed to parse command line: \"{}\"", exec).into());
            }
        }
    }
}

pub struct ExecCommandExecutor {}

#[async_trait]
impl ActionFn for ExecCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute ExecCommand");

        match from_value::<ExecCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action);
            }
            Err(err) => {
                return Err(format!("Failed to convert data to ExecCommand, err: {}", err).into());
            }
        }
    }
}
