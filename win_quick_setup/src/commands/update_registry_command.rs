use super::common::InstallActionType;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;

use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
use winreg::RegKey;
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

#[allow(dead_code)]
fn read_registry_key() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the HKEY_CURRENT_USER registry key
    let hklm = RegKey::predef(HKEY_CURRENT_USER);

    // Open a specific subkey (e.g., Software\\Microsoft\\Windows\\CurrentVersion)
    let subkey = hklm.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion", KEY_READ)?;

    // Read a specific value from the subkey (e.g., "ProgramFilesDir")
    let value: String = subkey.get_value("ProgramFilesDir")?;
    println!("Program Files directory: {}", value);

    Ok(())
}

#[allow(dead_code)]
fn write_registry_key() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the HKEY_CURRENT_USER registry key
    let hklm = RegKey::predef(HKEY_CURRENT_USER);

    // Create or open a specific subkey with write access
    let subkey = hklm.open_subkey_with_flags("Software\\MyApp", KEY_WRITE)?;

    // Set a new string value under the subkey
    subkey.set_value("MySetting", &"Hello, Registry!")?;

    println!("Value written successfully.");

    Ok(())
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