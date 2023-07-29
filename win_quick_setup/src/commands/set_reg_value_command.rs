use super::common::{expand_string_deserializer, InstallActionType, ActionFn};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::io;

use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
use winreg::RegKey;

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
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>> {
        match action {
            InstallActionType::INSTALL => {}
            _ => return Ok(true),
        }

        let hklm = RegKey::predef(HKEY_CURRENT_USER);

        let subkey = hklm.open_subkey_with_flags(&self.reg_path, KEY_WRITE)?;

        match subkey.delete_value(&self.key_name) {
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => {}
                _ => {
                    println!("Failed to delete registry value: {}", err);
                    return Ok(false);
                }
            },
            _ => {}
        }

        // I should expand everything on serialize to add trait for that!!!
        subkey.set_value(&self.key_name, &self.value.as_str())?;

        return Ok(true);
    }
}

pub struct UpdateRegistryCommandExecutor{
}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for UpdateRegistryCommandExecutor{
    async fn execute_command(&self, json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>>
    {
        let cmd: UpdateRegistryCommand = from_value(json_data.clone())?;

        return cmd.execute(action);
    }
}