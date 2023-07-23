use super::common::InstallActionType;

use serde_derive::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::error::Error;
use std::process::Command;

// if in the feature chocolatey will no longer be installed in env:ProgramData\\chocolatey this path will need to be changed
const REFRESHENV_COMMAND: &str ="Set-ExecutionPolicy Bypass -Scope Process; Import-Module $env:ProgramData\\chocolatey\\helpers\\chocolateyProfile.psm1;refreshenv;";

#[derive(Deserialize, Serialize)]
struct VcpkgCommand {
    module: String
}

impl VcpkgCommand {
    fn run_command(&self, exec: & String, args: & Vec<String>) -> Result<bool, Box<dyn Error>> {
        let exitcode: Option<i32>;

        exitcode = Command::new("powershell")
            .arg("-Command")
            .arg(REFRESHENV_COMMAND)
            .arg(exec)
            .args(args)
            .status()
            .map(|exitcode| exitcode.code())
            .unwrap_or(Some(-1));
     

        return Ok(exitcode.is_some_and(|x| x == 0));
    }
    
    pub fn execute(&self, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
        let mut exec: String = String::new();
        match action {
            InstallActionType::INSTALL => {
                exec.push_str("vcpkg install ");
            }
            InstallActionType::UNINSTALL => {
                exec.push_str("vcpkg uninstall ");
            }
            InstallActionType::UPDATE => {
                exec.push_str("vcpkg upgrade ");
            }
        }
        exec.push_str(self.module.as_str());
        
        println!("Executing command: {}", exec);

        if exec.len() == 0
        {
            return Ok(true);
        }
    
        return self.run_command(&exec, &Vec::new());
    }
}

pub fn vcpkg_command(json_data: &Value, action: &InstallActionType) -> Result<bool, Box<dyn Error>> {
    let cmd: VcpkgCommand = from_value(json_data.clone())?;

    return cmd.execute(action);
}
