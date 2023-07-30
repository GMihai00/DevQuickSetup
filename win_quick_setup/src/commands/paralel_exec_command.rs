use super::common::{ActionFn, InstallActionType};

use std::error::Error;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, json, Value};

use futures::future;
use tokio::task;

use log::debug;

use super::super::rendering::render;

#[derive(Deserialize, Serialize)]
struct ParalelExecCommand {
    run: Value,
}

impl ParalelExecCommand {
    fn handle_resolved_task<E: std::fmt::Debug>(
        &self,
        item_resolved: Result<Result<bool, Box<dyn Error + Send + Sync>>, E>,
        idx: usize,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match item_resolved {
            Ok(value) => match value {
                Ok(ret) => {
                    if !ret {
                        return Err(format!("Paralel task at index {} failed", idx).into());
                    }

                    return Ok(());
                }
                Err(err) => {
                    return Err(err);
                }
            },
            Err(err) => {
                return Err(format!("Task at index {} failed with error: {:?}", idx, err).into());
            }
        }
    }
    pub async fn execute(
        &self,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        if let Value::Array(obj) = &self.run {
            let mut tasks = vec![];
            for value in obj.iter() {
                if let Value::Object(object) = value {
                    if object.len() != 1 {
                        let json_string = serde_json::to_string(&object)
                            .expect("Failed to convert JSON to string");
                        return Err(format!("Invalid instruction found: {}", json_string).into());
                    }

                    let encapsulated_command = json!([object.clone()]);
                    let cpy_action = action.clone();
                    let task = task::spawn(async move {
                        return render(&encapsulated_command, &cpy_action).await;
                    });
                    tasks.push(task);
                }
            }

            let (item_resolved, idx, remaining_futures) = future::select_all(tasks).await;

            if let Err(err) = self.handle_resolved_task(item_resolved, idx) {
                return Err(err);
            }

            let mut futures = remaining_futures;

            while !futures.is_empty() {
                let (item_resolved, idx, remaining_futures) = future::select_all(futures).await;

                if let Err(err) = self.handle_resolved_task(item_resolved, idx) {
                    return Err(err);
                }

                futures = remaining_futures;
            }
        } else {
            let json_string =
                serde_json::to_string(&self.run).expect("Failed to convert JSON to string");
            return Err(format!("Invalid syntax, comands are supposed to be contained into an array of objects, found {}", json_string).into());
        }

        return Ok(true);
    }
}

pub struct ParalelExecCommandExecutor {}

use async_trait::async_trait;

#[async_trait]
impl ActionFn for ParalelExecCommandExecutor {
    async fn execute_command(
        &self,
        json_data: &Value,
        action: &InstallActionType,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Attempting to execute ParalelExecCommand");

        match from_value::<ParalelExecCommand>(json_data.clone()) {
            Ok(cmd) => {
                return cmd.execute(action).await;
            }
            Err(err) => {
                return Err(
                    format!("Failed to convert data to ParalelExecCommand, err: {}", err).into(),
                );
            }
        }
    }
}
