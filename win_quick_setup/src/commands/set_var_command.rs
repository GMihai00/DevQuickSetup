use super::common::{expand_string_deserializer, set_install_value, ActionFn, InstallActionType};

use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;

use log::debug;
#[derive(Deserialize, Serialize)]
struct SetVarCommand<T: Clone + Serialize> {
    key: String,
    value: T,
}

impl<T: Clone + Serialize> SetVarCommand<T> {
    pub fn execute(
        &self,
        _action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        set_install_value(&self.key.as_str(), self.value.clone());

        return Ok(true);
    }
}

#[derive(Deserialize, Serialize)]
struct SetStringVarCommand {
    key: String,
    #[serde(deserialize_with = "expand_string_deserializer")]
    value: String,
}

impl SetStringVarCommand {
    pub fn execute(
        &self,
        _action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        set_install_value(&self.key.as_str(), &self.value);

        return Ok(true);
    }
}

pub struct SetVarCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for SetVarCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute SetVarCommand");

        match json_data.get("value") {
            Some(value) => match value {
                Value::Bool(_) => match from_value::<SetVarCommand<bool>>(json_data.clone()) {
                    Ok(cmd) => {
                        return cmd.execute(action);
                    }
                    Err(err) => {
                        return Err(format!(
                            "Failed to convert data to SetVarCommand<bool>, err: {}",
                            err
                        )
                        .into());
                    }
                },
                Value::Number(_) => match from_value::<SetVarCommand<u32>>(json_data.clone()) {
                    Ok(cmd) => {
                        return cmd.execute(action);
                    }
                    Err(err) => {
                        return Err(format!(
                            "Failed to convert data to SetVarCommand<u32>, err: {}",
                            err
                        )
                        .into());
                    }
                },
                Value::String(_) => match from_value::<SetStringVarCommand>(json_data.clone()) {
                    Ok(cmd) => {
                        return cmd.execute(action);
                    }
                    Err(err) => {
                        return Err(format!(
                            "Failed to convert data to SetStringVarCommand, err: {}",
                            err
                        )
                        .into());
                    }
                },
                _ => {
                    return Err("Unsupported json data type found".into());
                }
            },
            _ => {
                return Err("Key 'value' not found in JSON data".into());
            }
        }
    }
}
