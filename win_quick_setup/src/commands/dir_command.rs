use super::common::{expand_string_deserializer, ActionFn, InstallActionType};

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::fs;

use log::debug;
#[derive(Deserialize, Serialize)]
struct DirCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    path: String,

    #[serde(default = "default_overwrite_option")]
    should_overwrite: bool,
}

fn default_overwrite_option() -> bool {
    return false;
}

impl DirCommand {
    pub fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        match action {
            InstallActionType::INSTALL => {}
            _ => {
                return Ok(true);
            }
        }

        if self.should_overwrite {
            match fs::remove_dir_all(&self.path) {
                Ok(_) => {}
                Err(err) => {
                    return Err(format!(
                        "Failed to cleanup directory: \"{}\" err: {}",
                        self.path, err
                    )
                    .into());
                }
            }
        }

        match fs::create_dir_all(&self.path) {
            Ok(()) => {
                return Ok(true);
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::AlreadyExists {
                    return Ok(true);
                } else {
                    return Err(format!(
                        "Failed to create directorie: \"{}\" err: {}",
                        self.path, err
                    )
                    .into());
                }
            }
        }
    }
}

pub struct DirCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for DirCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute DirCommand");

        match from_value::<DirCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action);
            }
            Err(err) => {
                return Err(format!("Failed to convert data to DirCommand, err: {}", err).into());
            }
        }
    }
}
