use super::commands;

use commands::common::InstallActionType;

use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use super::executor_factory::ExecutorFactory;

fn load_config_file(conf_file: &String) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match File::open(conf_file) {
        Ok(mut file) => {
            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => match serde_json::from_str::<Value>(&contents) {
                    Ok(json_data) => {
                        return Ok(json_data);
                    }
                    Err(err) => {
                        return Err(format!("Failed to parse json err: {}", err).into());
                    }
                },
                Err(err) => {
                    return Err(
                        format!("Failed to read file: \"{}\" err: {}", conf_file, err).into(),
                    );
                }
            }
        }
        Err(err) => {
            return Err(format!("Failed to open file: \"{}\", err: {}", conf_file, err).into());
        }
    }
}

pub async fn install_config(
    conf_file: &String,
    action: &InstallActionType,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    match load_config_file(&conf_file.clone()) {
        Ok(json_data) => {
            return render(&json_data, &action).await;
        }
        Err(err) => {
            return Err(format!("Failed to load config err: {}", err).into());
        }
    }
}

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

                    return Err(format!("Invalid instruction found: {}", json_string).into());
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
