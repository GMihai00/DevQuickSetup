use lazy_static::lazy_static;
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde::{Deserialize, Deserializer};
use serde_json::{json, Value};
use std::error::Error;
use std::option::Option;
use std::sync::Mutex;

use log::warn;

// if in the feature chocolatey will no longer be installed in env:ProgramData\\chocolatey this path will need to be changed
pub const REFRESHENV_COMMAND: &str ="Set-ExecutionPolicy Bypass -Scope Process; Import-Module $env:ProgramData\\chocolatey\\helpers\\chocolateyProfile.psm1;refreshenv;";
#[derive(Clone)]
pub enum InstallActionType {
    INSTALL,
    UNINSTALL,
    UPDATE,
}

use async_trait::async_trait;

#[async_trait]
pub trait ActionFn {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

lazy_static! {
    static ref INSTALL_VALUES: Mutex<serde_json::Value> = Mutex::new(json!({}));
}

pub fn get_install_value<T: DeserializeOwned>(key: &str) -> Option<T> {
    let install_vals = INSTALL_VALUES.lock().unwrap();

    if install_vals.get(key).is_some() {
        let val = &install_vals[key];

        match serde_json::from_value(val.clone()) {
            Ok(val) => return Some(val),
            Err(_err) => {}
        };
    }

    return None;
}

pub fn set_install_value<T: Serialize>(key: &str, value: T) {
    let mut install_vals = INSTALL_VALUES.lock().unwrap();

    install_vals[key] = json!(value);
}

pub fn expand_string(input_string: &str) -> String {
    let re = Regex::new(r"%(.*?)%").unwrap();

    let result = re.replace_all(input_string, |caps: &regex::Captures| {
        let captured_value = caps.get(1).map_or("", |m| m.as_str());

        let install_val: Option<String> = get_install_value(captured_value);
        match install_val {
            Some(val) => val,
            None => {
                let install_val: Option<u32> = get_install_value(captured_value);
                match install_val {
                    Some(val) => val.to_string(),
                    None => {
                        warn!("Failed to find install value: \"{}\"", captured_value);
                        return captured_value.to_owned().to_string();
                    }
                }
            }
        }
    });

    return result.to_string();
}

pub fn expand_string_deserializer<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_value: String = Deserialize::deserialize(deserializer)?;

    Ok(expand_string(&raw_value.as_str()))
}
