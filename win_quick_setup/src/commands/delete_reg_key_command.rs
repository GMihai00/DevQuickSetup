use super::common::{expand_string_deserializer, InstallActionType, ActionFn};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::io;

use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS};
use winreg::RegKey;

#[derive(Deserialize, Serialize)]
struct DeleteRegistryValueCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    reg_path: String,

    #[serde(deserialize_with = "expand_string_deserializer")]
    key_name: String,
}

impl DeleteRegistryValueCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>> {
        match action {
            InstallActionType::INSTALL => {}
            _ => return Ok(true),
        }

        let hklm = RegKey::predef(HKEY_CURRENT_USER);

        match hklm.open_subkey_with_flags(&self.reg_path, KEY_ALL_ACCESS) {
            Ok(subkey) => match subkey.delete_value(&self.key_name) {
                Err(err) => match err.kind() {
                    io::ErrorKind::NotFound => {}
                    _ => {
                        println!("Failed to delete registry value: {}", err);
                        return Ok(false);
                    }
                },
                _ => {}
            },
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {}
                _ => {
                    println!("Failed to open registry key err : {}", err);
                    return Ok(false);
                }
            },
        }

        return Ok(true);
    }
}

pub struct DeleteRegistryValueCommandExecutor{
}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for DeleteRegistryValueCommandExecutor
{
    async fn execute_command(&self, json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>>
    {
        let cmd: DeleteRegistryValueCommand = from_value(json_data.clone())?;

        return cmd.execute(action);
    }
}