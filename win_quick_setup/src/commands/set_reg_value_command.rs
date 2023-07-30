use super::common::{expand_string_deserializer, ActionFn, InstallActionType};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;

use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
use winreg::RegKey;

use log::debug;
#[derive(Deserialize, Serialize)]
struct UpdateRegistryCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    reg_path: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    key_name: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    value: String,
}

impl UpdateRegistryCommand {
    pub fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        match action {
            InstallActionType::INSTALL => {}
            _ => return Ok(true),
        }

        let hklm = RegKey::predef(HKEY_CURRENT_USER);

        match hklm.open_subkey_with_flags(&self.reg_path, KEY_WRITE) {
            Ok(subkey) => match subkey.set_value(&self.key_name, &self.value.as_str()) {
                Ok(_) => {
                    return Ok(true);
                }
                Err(err) => {
                    return Err(format!(
                        "Failed to delete registry key: \"{}\" path: \"{}\" err: {}",
                        self.key_name, self.reg_path, err
                    )
                    .into());
                }
            },
            Err(err) => {
                return Err(format!(
                    "Failed to open registry key: \"{}\" path: \"{}\" err: {}",
                    self.key_name, self.reg_path, err
                )
                .into());
            }
        }
    }
}

pub struct UpdateRegistryCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for UpdateRegistryCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute UpdateRegistryCommand");

        match from_value::<UpdateRegistryCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action);
            }
            Err(err) => {
                return Err(format!(
                    "Failed to convert data to UpdateRegistryCommand, err: {}",
                    err
                )
                .into());
            }
        }
    }
}
