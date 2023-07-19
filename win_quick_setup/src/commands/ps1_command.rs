use super::common::InstallActionType;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::env;
use std::error::Error;
use std::process::Command;
#[derive(Deserialize, Serialize)]
struct PowershellCommand {
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

impl PowershellCommand {
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
        
        let (exec, args) = shell_words::split(&exec)
        .map(|parsed| {
            parsed.split_first()
                .map(|(exec, args)| (exec.to_string(), args.to_vec()))
        })
        .unwrap_or_else(|err| {
            eprintln!("Error parsing command: {:?}", err);
            panic!("Failed to parse cmdline {}", &exec.as_str());
        })
        .expect("Invalid command.");    
    
        let exitcode = Command::new("powershell")
            .arg("-Command")
            .arg(exec)
            .args(args)
            .current_dir(env::current_dir()?)
            .status()
            .map(|exitcode| exitcode.code())
            .unwrap_or(Some(-1));

        return Ok(exitcode.is_some_and(|x| x == 0));
    }
}

pub fn run_ps1_command(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: PowershellCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}
