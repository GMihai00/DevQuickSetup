use super::common::InstallActionType;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;

#[derive(Deserialize, Serialize)]
struct UpdateRegistryCommand {
    registry_path: String,
    key_name: String,
    #[serde(default = "default_added_string_value")]
    added_string_value: String,
    #[serde(default = "default_added_int_value")]
    added_int_value: i32,
    #[serde(default = "default_should_overwrite")]
    should_overwrite: bool
}

fn default_added_string_value() -> String {
    return String::new();
}

fn default_added_int_value() -> i32 {
    return 0;
}

fn default_should_overwrite() -> bool {
    return false;
}

impl UpdateRegistryCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
         // TODO: Implement this function
        unimplemented!();
    }
}

// AR TREBUI SA SCHIMB FUNCTIA ASTA SA FIE OVERALL ALEASA, SA POT SA-I SPECIFIC DOAR TIPUL DE DATE LA CMD CA IN REST SUNT CAM 
// THE SAME THING SA VAD DACA POT ALEGE DINAMIC TIPURI DE DATE LA RUNTIME
pub fn update_registry(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: UpdateRegistryCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}