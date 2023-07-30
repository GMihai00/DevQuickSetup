use super::common::{
    expand_string, expand_string_deserializer, set_install_value, ActionFn, InstallActionType,
};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;

use winreg::enums::{HKEY_CURRENT_USER, KEY_READ};
use winreg::RegKey;

use log::{debug, warn};
#[derive(Deserialize, Serialize)]
struct GetRegistryValueCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    reg_path: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    key_name: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    install_key: String,

    #[serde(default = "default_can_fail_option")]
    can_fail: bool,
}

fn default_can_fail_option() -> bool {
    return false;
}

impl GetRegistryValueCommand {
    fn handle_err_case(&self, err: std::io::Error) -> Result<bool, Box<dyn Error + Send + Sync>> {
        if !self.can_fail {
            return Err(format!(
                "Failed to get registry value, key: \"{}\" path: \"{}\" err {}",
                self.key_name, self.reg_path, err
            )
            .into());
        } else {
            warn!(
                "Failed to get registry value, key: \"{}\" path: \"{}\" err {}",
                self.key_name, self.reg_path, err
            );
            return Ok(true);
        }
    }

    pub fn execute(
        &self,
        _action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let hklm = RegKey::predef(HKEY_CURRENT_USER);

        match hklm.open_subkey_with_flags(&self.reg_path.as_str(), KEY_READ) {
            Ok(subkey) => match subkey.get_value::<String, _>(&self.key_name.as_str()) {
                Ok(string_value) => {
                    set_install_value(
                        &self.install_key.as_str(),
                        expand_string(string_value.as_str()).as_str(),
                    );
                }
                Err(_) => match subkey.get_value::<u32, _>(&self.key_name.as_str()) {
                    Ok(dword_value) => set_install_value(&self.install_key.as_str(), dword_value),
                    Err(err) => {
                        return self.handle_err_case(err);
                    }
                },
            },
            Err(err) => {
                return self.handle_err_case(err);
            }
        }

        return Ok(true);
    }
}

pub struct GetRegistryValueCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for GetRegistryValueCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute GetRegistryValueCommand");

        match from_value::<GetRegistryValueCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action);
            }
            Err(err) => {
                return Err(format!(
                    "Failed to convert data to GetRegistryValueCommand, err: {}",
                    err
                )
                .into());
            }
        }
    }
}
