use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::process::Command;

pub enum InstallActionType {
    INSTALL,
    UNINSTALL,
    UPDATE,
}
#[derive(Deserialize, Serialize)]
struct JsonCommand {
    install_run: String,
    uninstall_run: String,
    update_run: String,
}

impl JsonCommand {
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        let exec: &String;
        match action {
            InstallActionType::INSTALL => {
                exec = &self.install_run;
            }
            InstallActionType::UNINSTALL => {
                exec = &self.uninstall_run;
            }
            InstallActionType::UPDATE => {
                exec = &self.update_run;
            }
        }

        println!("Executing command: {}", exec);

        let exitcode: Option<i32> = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &exec.as_str()])
                .status()
                .map(|exitcode| exitcode.code())
                .unwrap_or(Some(-1))
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&exec.as_str())
                .status()
                .map(|exitcode| exitcode.code())
                .unwrap_or(Some(-1))
        };

        if exec.starts_with("winget") {
            // DUE TO WRONG EXIT CODE WHEN NO NEW PACKAGE FOUND...
            // SHOULD AT LEAST BE SOMETHING LIKE
            return Ok(true);
        }

        return Ok(exitcode.is_some_and(|x| x == 0));
    }
}

type ActionFn = fn(&Value, &InstallActionType) -> Result<bool, Box<dyn Error>>;

fn run_command(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: JsonCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}

const ACTION_MAP: &[(&str, ActionFn); 1] = &[("exec", run_command)];

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
