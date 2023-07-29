use super::commands;

use std::error::Error;
use serde_json::Value;

use commands::common::{ ActionFn, InstallActionType};
use commands::delete_reg_key_command::delete_reg_key;
use commands::dir_command::create_dir;
use commands::exec_command::run_command;
use commands::get_reg_value_command::get_registry_value;
use commands::ps1_command::run_ps1_command;
use commands::set_reg_value_command::update_registry;
use commands::set_var_command::set_install_var;
use commands::vcpkg_command::vcpkg_command;
use commands::winget_command::winget_run;
use commands::include_command::include;
use commands::conditional_command::check_condition;

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
