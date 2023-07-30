use super::common::{expand_string_deserializer, ActionFn, InstallActionType, REFRESHENV_COMMAND};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::process::Command;

use log::debug;

#[derive(Deserialize, Serialize)]
struct VcpkgCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    module: String,
}

impl VcpkgCommand {
    fn run_command(
        &self,
        exec: &String,
        args: &Vec<String>,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let exitcode: Option<i32>;

        exitcode = Command::new("powershell")
            .arg("-Command")
            .arg(REFRESHENV_COMMAND)
            .arg(exec)
            .args(args)
            .status()
            .map(|exitcode| exitcode.code())
            .unwrap_or(Some(-1));

        return Ok(exitcode.is_some_and(|x| x == 0));
    }

    pub fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let mut exec: String = String::new();
        match action {
            InstallActionType::INSTALL => {
                exec.push_str("vcpkg install ");
            }
            InstallActionType::UNINSTALL => {
                exec.push_str("vcpkg uninstall ");
            }
            InstallActionType::UPDATE => {
                exec.push_str("vcpkg upgrade ");
            }
        }
        exec.push_str(self.module.as_str());

        println!("Executing command: \"{}\"", exec);

        if exec.len() == 0 {
            return Ok(true);
        }

        return self.run_command(&exec, &Vec::new());
    }
}

pub struct VcpkgCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for VcpkgCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute VcpkgCommand");

        match from_value::<VcpkgCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action);
            }
            Err(err) => {
                return Err(format!("Failed to convert data to VcpkgCommand, err: {}", err).into());
            }
        }
    }
}
