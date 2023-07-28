use super::common::{InstallActionType, expand_string_deserializer};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::io;

use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
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

impl UpdateRegistryCommand {
    
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        
        match action {
            InstallActionType::INSTALL => {}
            _ => { return Ok(true) }
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
            }
            _ => {}
        }
    
        // I should expand everything on serialize to add trait for that!!!
        subkey.set_value(&self.key_name, &self.value.as_str())?;   
        
        return Ok(true);
    }
}

// AR TREBUI SA SCHIMB FUNCTIA ASTA SA FIE OVERALL ALEASA, SA POT SA-I SPECIFIC DOAR TIPUL DE DATE LA CMD CA IN REST SUNT CAM 
// THE SAME THING SA VAD DACA POT ALEGE DINAMIC TIPURI DE DATE LA RUNTIME
pub fn update_registry(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: UpdateRegistryCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}