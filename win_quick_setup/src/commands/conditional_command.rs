use super::common::{expand_string_deserializer, ActionFn, InstallActionType};

use std::error::Error;

use log::debug;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, json, Value};

use regex::Regex;

use super::super::rendering::render;

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
    pub async fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Checking condition: {}", self.condition);

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
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                ">=" => {
                    if value1 >= value2 {
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                "<=" => {
                    if value1 <= value2 {
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                ">" => {
                    if value1 > value2 {
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                "<" => {
                    if value1 < value2 {
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                "!=" => {
                    if value1 != value2 {
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                "contains" => {
                    if let Some(_) = value1.find(value2) {
                        return render(&self.run, &action).await;
                    } else {
                        return render(&self.except, &action).await;
                    }
                }
                "!contains" => {
                    if let Some(_) = value1.find(value2) {
                        return render(&self.except, &action).await;
                    } else {
                        return render(&self.run, &action).await;
                    }
                }
                _ => {
                    return Err(format!(
                        "Internal error, {} not a valid comparison operator",
                        operator
                    )
                    .into());
                }
            }
        } else {
            return Err(format!(
                "Invalid if statement, condition \"{}\" doesn't match pattern \"{}\"",
                self.condition, pattern
            )
            .into());
        }
    }
}

pub struct ConditionalCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for ConditionalCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute ConditionalCommand");

        match from_value::<ConditionalCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action).await;
            }
            Err(err) => {
                return Err(
                    format!("Failed to convert data to ConditionalCommand, err: {}", err).into(),
                );
            }
        }
    }
}
