use super::commands;

use commands::common::InstallActionType;

use serde_json::Value;
use std::error::Error;

use super::executor_factory::ExecutorFactory;

pub async fn render(
    json_data: &Value,
    action: &InstallActionType,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
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
                    let executor = ExecutorFactory::build(first_key.as_str());
                    let future = executor.execute_command(&object[first_key], action);

                    match future.await {
                        Ok(ret) => {
                            if !ret {
                                let json_string = serde_json::to_string(&object)
                                    .expect("Failed to convert JSON to string");
                                println!("Command failed: {}", json_string);
                                return Ok(false);
                            }
                        }
                        Err(err) => {
                            let json_string = serde_json::to_string(&object)
                                .expect("Failed to convert JSON to string");
                            panic!(
                                "Failed to run command: \"{}\", err: \"{}\"",
                                json_string, err
                            );
                        }
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
    } else {
        let json_string =
            serde_json::to_string(&json_data).expect("Failed to convert JSON to string");
        panic!("Invalid syntax, comands are supposed to be contained into an array of objects, found {}", json_string);
    }

    return Ok(true);
}
