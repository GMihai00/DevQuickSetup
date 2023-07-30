use super::common::{expand_string_deserializer, ActionFn, InstallActionType, REFRESHENV_COMMAND};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::env;
use std::error::Error;
use std::process::Command;

use log::{debug, warn};

#[derive(Deserialize, Serialize)]
struct PowershellCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    install_run: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    #[serde(default = "default_uninstall_run")]
    uninstall_run: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    #[serde(default = "default_update_run")]
    update_run: String,

    #[serde(default = "default_refresh_env")]
    refresh_env: bool,

    #[serde(default = "default_preparse")]
    preparse: bool,

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

fn default_refresh_env() -> bool {
    return false;
}

fn default_preparse() -> bool {
    return true;
}

fn default_dir() -> String {
    if let Ok(current_dir) = env::current_dir() {
        return current_dir.to_string_lossy().to_string();
    } else {
        warn!("Failed to get current directory");
        return String::new();
    }
}

impl PowershellCommand {
    fn run_command(
        &self,
        exec: &String,
        args: &Vec<String>,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exitcode: Option<i32>;
        if self.refresh_env {
            exitcode = Command::new("powershell")
                .arg("-Command")
                .arg(REFRESHENV_COMMAND)
                .arg(exec)
                .args(args)
                .current_dir(&self.dir)
                .status()
                .map(|exitcode| exitcode.code())
                .unwrap_or(Some(-1));
        } else {
            exitcode = Command::new("powershell")
                .arg("-Command")
                .arg(exec)
                .args(args)
                .current_dir(&self.dir)
                .status()
                .map(|exitcode| exitcode.code())
                .unwrap_or(Some(-1));
        }

        return Ok(exitcode.is_some_and(|x| x == 0));
    }

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

        println!(
            "Executing command: \"{}\" refresh_emv: {}",
            exec, self.refresh_env
        );

        if exec.len() == 0 {
            return Ok(true);
        }

        if self.preparse {
            match shell_words::split(&exec).map(|parsed| {
                parsed
                    .split_first()
                    .map(|(exec, args)| (exec.to_string(), args.to_vec()))
            }) {
                Ok(Some((exec, args))) => {
                    return self.run_command(&exec, &args);
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

        return self.run_command(&exec, &Vec::new());
    }
}
pub struct PowershellCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for PowershellCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute PowershellCommand");

        match from_value::<PowershellCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action);
            }
            Err(err) => {
                return Err(
                    format!("Failed to convert data to PowershellCommand, err: {}", err).into(),
                );
            }
        }
    }
}
