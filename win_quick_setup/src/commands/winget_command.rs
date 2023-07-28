use super::common::{expand_string_deserializer, InstallActionType};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::process::Command;

#[derive(Deserialize, Serialize)]
struct WingetCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    package: String,
}

impl WingetCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        let mut exec: String = String::from("winget ");
        match action {
            InstallActionType::INSTALL => {
                exec.push_str("install");
            }
            InstallActionType::UNINSTALL => {
                exec.push_str("uninstall");
            }
            InstallActionType::UPDATE => {
                exec.push_str("update");
            }
        }
        exec.push_str(" --accept-package-agreements ");
        exec.push_str(self.package.as_str());

        println!("Executing command: {}", exec);

        if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", &exec.as_str()]).status()?;
        } else {
            panic!("Winget command not allowed on OS other then windows");
        };

        return Ok(true);
    }
}

pub fn winget_run(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: WingetCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}
