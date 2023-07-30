use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

use super::super::rendering::render;

use super::common::{expand_string_deserializer, get_install_value, ActionFn, InstallActionType};

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
            let conf_dir: Option<String> = get_install_value("CONF_DIR");

            match conf_dir {
                Some(conf_dir) => {
                    config_path = conf_dir.to_string() + self.config_path.as_str();
                }
                None => {
                    panic!("Failed to find config dir")
                }
            }
        }

        let mut file =
            File::open(&config_path).expect(&format!("Failed to open file {}", config_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

        return render(&json_data, &action).await;
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
        let cmd: IncludeCommand = from_value(json_data.clone())?;
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
}
