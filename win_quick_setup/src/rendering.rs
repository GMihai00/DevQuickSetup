use super::commands;

use serde_json::{from_value, json, Value};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use commands::common::{
    expand_string_deserializer, get_install_value, ActionFn, InstallActionType,
};
use commands::delete_reg_key_command::delete_reg_key;
use commands::dir_command::create_dir;
use commands::exec_command::run_command;
use commands::get_reg_value_command::get_registry_value;
use commands::ps1_command::run_ps1_command;
use commands::set_reg_value_command::update_registry;
use commands::set_var_command::set_install_var;
use commands::vcpkg_command::vcpkg_command;
use commands::winget_command::winget_run;

use serde_derive::{Deserialize, Serialize};

use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;

use regex::Regex;

lazy_static! {
    static ref USED_CONFIG_PATHS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

const ACTION_MAP: &[(&str, ActionFn); 12] = &[
    ("exec", run_command),
    ("winget", winget_run),
    ("include", include),
    ("reg_update", update_registry),
    ("ps1", run_ps1_command),
    ("vcpkg", vcpkg_command),
    ("dir", create_dir),
    ("set_reg_val", update_registry),
    ("set_var", set_install_var),
    ("get_reg_val", get_registry_value),
    ("if", check_condition),
    ("delete_reg_key", delete_reg_key),
];
#[derive(Deserialize, Serialize)]
struct IncludeCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    config_path: String,
}

impl IncludeCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
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

        return render(&json_data, &action);
    }
}

fn include(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: IncludeCommand = from_value(json_data.clone())?;

    {
        let config_path = cmd.config_path.clone();
        let mut used_paths = USED_CONFIG_PATHS.lock().unwrap();
        if used_paths.contains(&config_path) {
            return Ok(true);
        }
        used_paths.insert(config_path);
    }

    return cmd.execute(action);
}

#[derive(Deserialize, Serialize)]
struct ConditionalCommand {
    #[serde(deserialize_with = "expand_string_deserializer")]
    condition: String,

    #[serde(default = "default_run")]
    run: Value,

    #[serde(rename = "else")]
    #[serde(default = "default_except_run")]
    except: Value,
}

fn default_run() -> Value {
    return json!([]);
}

fn default_except_run() -> Value {
    return json!([]);
}

impl ConditionalCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        let pattern = r"(.+)\s*(==|>=|<=|!=|<|>|contains|!contains)\s*(.+)";
        let re = Regex::new(pattern).unwrap();

        if let Some(captures) = re.captures(&self.condition) {
            let value1 = captures.get(1).unwrap().as_str();
            let operator = captures.get(2).unwrap().as_str();
            let value2 = captures.get(3).unwrap().as_str();

            let value1 = value1.trim_start().trim_end();
            let value2 = value2.trim_start().trim_end();

            match operator {
                "==" => {
                    if value1 == value2 {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                ">=" => {
                    if value1 >= value2 {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                "<=" => {
                    if value1 <= value2 {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                ">" => {
                    if value1 > value2 {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                "<" => {
                    if value1 < value2 {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                "!=" => {
                    if value1 != value2 {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                "contains" => {
                    if let Some(_) = value1.find(value2) {
                        return render(&self.run, &action);
                    } else {
                        return render(&self.except, &action);
                    }
                }
                "!contains" => {
                    if let Some(_) = value1.find(value2) {
                        return render(&self.except, &action);
                    } else {
                        return render(&self.run, &action);
                    }
                }
                _ => {
                    return Err("Internal error".into());
                }
            }
        } else {
            return Err("No match found, invalid if statement!".into());
        }
    }
}

pub fn check_condition(
    json_data: &Value,
    action: &InstallActionType,
) -> Result<bool, Box<dyn Error>> {
    let cmd: ConditionalCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}

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
