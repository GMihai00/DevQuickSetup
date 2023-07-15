mod rendering;

use std::error::Error;
use serde_json::json;

use crate::rendering::run_command;

fn main() -> Result<(), Box<dyn Error>> {
    
    run_command(json!({ "exec": "winget install BurntSushi.ripgrep.MSVC" }))?;
    
    Ok(())
}
