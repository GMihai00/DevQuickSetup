use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::path::Path;

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

use log::debug;

use super::common::{expand_string_deserializer, get_install_value, ActionFn, InstallActionType};

use super::super::rendering::install_config;

lazy_static! {
    static ref INCLUDED_CONFIGS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

#[derive(Deserialize, Serialize)]
struct IncludeCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    config_path: String,
}

impl IncludeCommand {
    pub async fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let path = self.config_path.clone();
        let path = Path::new(path.as_str());

        let mut config_path = self.config_path.clone();

        if !(path.is_file() && path.is_absolute()) {
            match get_install_value::<String>("CONF_DIR") {
                Some(conf_dir) => {
                    config_path = conf_dir.to_string() + self.config_path.as_str();
                }
                None => {
                    return Err("Failed to find config dir".into());
                }
            }
        }

        return install_config(&config_path, action).await;
    }
}

pub struct IncludeCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for IncludeCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute IncludeCommandExecutor");

        match from_value::<IncludeCommand>(json_data.clone()) {
            Ok(cmd) => {
                {
                    let config_path = cmd.config_path.clone();
                    let mut used_paths = INCLUDED_CONFIGS.lock().unwrap();
                    if used_paths.contains(&config_path) {
                        return Ok(true);
                    }
                    used_paths.insert(config_path);
                }

                return cmd.execute(action).await;
            }
            Err(err) => {
                return Err(format!(
                    "Failed to convert data to IncludeCommandExecutor, err: {}",
                    err
                )
                .into());
            }
        }
    }
}
