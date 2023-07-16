
use super::commands;

use serde_json::Value;
use std::error::Error;
use commands::common::{ActionFn, InstallActionType};
use commands::exec_command::run_command;

const ACTION_MAP: &[(&str, ActionFn); 1] = &[
    ("exec", run_command)
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
