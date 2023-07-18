use super::common::InstallActionType;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::process::Command;

#[derive(Deserialize, Serialize)]
struct ExecCommand {
    install_run: String,
    #[serde(default = "default_uninstall_run")]
    uninstall_run: String,
    #[serde(default = "default_update_run")]
    update_run: String,
}

fn default_uninstall_run() -> String {
    return String::new();
}

fn default_update_run() -> String {
    return String::new();
}

impl ExecCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
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

        println!("Executing command: {}", exec);

        if exec.len() == 0
        {
            return Ok(true);
        }

        let exitcode: Option<i32> = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &exec.as_str()])
                .status()
                .map(|exitcode| exitcode.code())
                .unwrap_or(Some(-1))
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&exec.as_str())
                .status()
                .map(|exitcode| exitcode.code())
                .unwrap_or(Some(-1))
        };

        if exec.starts_with("winget") {
            // DUE TO WRONG EXIT CODE WHEN NO NEW PACKAGE FOUND...
            // SHOULD AT LEAST BE SOMETHING LIKE
            return Ok(true);
        }

        return Ok(exitcode.is_some_and(|x| x == 0));
    }
}

pub fn run_command(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: ExecCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}
