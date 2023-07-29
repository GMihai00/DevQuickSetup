use super::common::{expand_string_deserializer, InstallActionType, ActionFn};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::env;
use std::error::Error;
use std::process::Command;

// if in the feature chocolatey will no longer be installed in env:ProgramData\\chocolatey this path will need to be changed
const REFRESHENV_COMMAND: &str ="Set-ExecutionPolicy Bypass -Scope Process; Import-Module $env:ProgramData\\chocolatey\\helpers\\chocolateyProfile.psm1;refreshenv;";

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
        return String::new();
    }
}

impl PowershellCommand {
    fn run_command(&self, exec: &String, args: &Vec<String>) -> Result<bool, Box<dyn Error  + Send + Sync>> {
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

    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>> {
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
            "Executing command: {} refresh_emv: {}",
            exec, self.refresh_env
        );

        if exec.len() == 0 {
            return Ok(true);
        }

        if self.preparse {
            let (exec, args) = shell_words::split(&exec)
                .map(|parsed| {
                    parsed
                        .split_first()
                        .map(|(exec, args)| (exec.to_string(), args.to_vec()))
                })
                .unwrap_or_else(|err| {
                    eprintln!("Error parsing command: {:?}", err);
                    panic!("Failed to parse cmdline {}", &exec.as_str());
                })
                .expect("Invalid command.");

            return self.run_command(&exec, &args);
        }

        return self.run_command(&exec, &Vec::new());
    }
}
pub struct PowershellCommandExecutor{
}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for PowershellCommandExecutor{
    async fn execute_command(&self, json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>>
    {
        let cmd: PowershellCommand = from_value(json_data.clone())?;

        return cmd.execute(action);
    }
}