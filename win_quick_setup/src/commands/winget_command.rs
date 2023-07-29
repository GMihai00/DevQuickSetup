use super::common::{expand_string_deserializer, InstallActionType, ActionFn};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::process::Command;

use async_trait::async_trait;
#[derive(Deserialize, Serialize)]
struct WingetCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    package: String,
}

impl WingetCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>> {
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


pub struct WingetCommandExecutor{
}

#[async_trait]
impl ActionFn for WingetCommandExecutor {
    async fn execute_command(&self, json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>> {
        let cmd: WingetCommand = from_value(json_data.clone())?;
    
        return cmd.execute(action);
    }
}