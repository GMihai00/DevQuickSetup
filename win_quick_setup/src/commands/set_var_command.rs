use super::common::{expand_string_deserializer, set_install_value, InstallActionType};

use serde::Serialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
#[derive(Deserialize, Serialize)]
struct SetVarCommand<T: Clone + Serialize> {
    key: String,
    value: T,
}

impl<T: Clone + Serialize> SetVarCommand<T> {
    pub fn execute(&self, _action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
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
    pub fn execute(&self, _action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        set_install_value(&self.key.as_str(), &self.value);

        return Ok(true);
    }
}

pub fn set_install_var(
    json_data: &Value,
    action: &InstallActionType,
) -> Result<bool, Box<dyn Error>> {
    match json_data.get("value") {
        Some(value) => match value {
            Value::Bool(_) => {
                let cmd: SetVarCommand<bool> = from_value(json_data.clone())?;

                return cmd.execute(action);
            }
            Value::Number(_) => {
                let cmd: SetVarCommand<u32> = from_value(json_data.clone())?;

                return cmd.execute(action);
            }
            Value::String(_) => {
                let cmd: SetStringVarCommand = from_value(json_data.clone())?;

                return cmd.execute(action);
            }
            _ => {
                return Err("Unsupported data type".into());
            }
        },
        _ => {
            return Err("Key 'value' not found in JSON data".into());
        }
    }
}
