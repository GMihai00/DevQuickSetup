use super::common::InstallActionType;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::fs;

#[derive(Deserialize, Serialize)]
struct DirCommand {
    path: String,
    #[serde(default = "default_overwrite_option")]
    should_overwrite: bool
}

fn default_overwrite_option() -> bool {
    return false;
}

impl DirCommand {
    fn cleanup(&self) -> bool {
    
        match fs::remove_dir_all(&self.path) {
            Ok(()) => {
                return true;
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return true;
                } else {
                    eprintln!("Failed to delete directory: {}", err);
                    return false;
                }
            }
        }
    }

    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        match action {
            InstallActionType::INSTALL => {
                if self.should_overwrite {
            
                    let ret = self.cleanup();
                    
                    if  ret == false{
                        println!("Failed to cleanup directory");
                        return Ok(false);
                    }
                }
                
                match fs::create_dir_all(&self.path) {
                    Ok(()) => {
                        return Ok(true);
                    }
                    Err(err) => {
                        if err.kind() == std::io::ErrorKind::AlreadyExists {
                            return  Ok(true);
                        }
                        else {
                            eprintln!("Failed to create directories: {}", err);
                            return Ok(false);
                        }
                    }
                }
                
            }
            _ => {
                return Ok(true);
            }
        }
        
    }
}

pub fn create_dir(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: DirCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}
