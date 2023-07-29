use super::common::{expand_string_deserializer,  InstallActionType, ActionFn};

use std::error::Error;

use serde_json::{from_value, json, Value};
use serde_derive::{Deserialize, Serialize};

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
    pub async fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>> {
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
                    return Err("Internal error".into());
                }
            }
        } else {
            return Err("No match found, invalid if statement!".into());
        }
    }
}

pub struct ConditionalCommandExecutor{
}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for ConditionalCommandExecutor {
    async fn execute_command(&self, json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error  + Send + Sync>>{
        let cmd: ConditionalCommand = from_value(json_data.clone())?;

        return cmd.execute(action).await;
    }
}