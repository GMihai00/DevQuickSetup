
use super::commands;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use serde_json::{from_value,Value};

use commands::common::{ActionFn, InstallActionType};
use commands::exec_command::run_command;
use commands::winget_command::winget_run;
use commands::update_registry_command::update_registry;

use serde_derive::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]

struct IncludeCommand {
    config_path: String
}

impl IncludeCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        let mut file = File::open(&self.config_path).expect(&format!("Failed to open file {}", &self.config_path));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
    
        let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");
    
        return render(&json_data, &action);
    }
}

fn include(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: IncludeCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}


const ACTION_MAP: &[(&str, ActionFn); 4] = &[
    ("exec", run_command),
    ("winget", winget_run),
    ("include", include),
    ("reg_update", update_registry)
];

pub fn render(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    if let Value::Array(obj) = json_data {
        for value in obj.iter() {
            if let Value::Object(object) = value {
                if object.len() != 1 {
                    let json_string =
                        serde_json::to_string(&object).expect("Failed to convert JSON to string");
                    panic!(
                        "Failed to found matching instruction for json: {}",
                        json_string
                    );
                }

                if let Some(first_key) = object.keys().next() {
                    if let Some(&function) = ACTION_MAP
                        .iter()
                        .find(|&(key, _)| key == first_key)
                        .map(|(_, function)| function)
                    {
                        let ok = (function)(&object[first_key], action)?;
                        if !ok {
                            println!("One of the instructions failed, halting execution");
                            return Ok(false);
                        }
                    } else {
                        let json_string = serde_json::to_string(&object)
                            .expect("Failed to convert JSON to string");
                        panic!(
                            "Failed to found matching instruction for json: {}",
                            json_string
                        );
                    }
                } else {
                    let json_string =
                        serde_json::to_string(&object).expect("Failed to convert JSON to string");
                    panic!(
                        "Failed to found matching instruction for json: {}",
                        json_string
                    );
                }
            }
        }
    }

    return Ok(true);
}
