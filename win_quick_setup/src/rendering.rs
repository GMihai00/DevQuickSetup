use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use serde_json::{Value, from_value};
use std::process::Command;

#[derive(Deserialize, Serialize)]
struct JsonCommand {
    exec: String,
}

impl JsonCommand {
    pub fn execute(&self) -> Result<bool, Box<dyn Error>> {
    
        println!("Executing command: {}", self.exec);
        
        let succes: bool = if cfg!(target_os = "windows") {
            Command::new("cmd")
                    .args(&["/C", self.exec.as_str()])
                    .status()
                    .map(|status| status.success())
                    .unwrap_or(false)
        } else {
            Command::new("sh")
                    .arg("-c")
                    .arg(&self.exec.as_str())
                    .status()
                    .map(|status| status.success())
                    .unwrap_or(false)
        };
        
        return Ok(succes);
    }
}

pub fn run_command(json_data: Value) -> Result<bool, Box<dyn Error>> {
    let cmd: JsonCommand = from_value(json_data)?;
    
    return cmd.execute();
}