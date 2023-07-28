mod commands;
mod rendering;

use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

use commands::common::set_install_value;
use commands::common::InstallActionType;
use rendering::render;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let action: InstallActionType;
        match &args[1].as_str() {
            &"--install" => action = InstallActionType::INSTALL,
            &"--uninstall" => action = InstallActionType::UNINSTALL,
            &"--update" => action = InstallActionType::UPDATE,
            _ => panic!("Invalid option passed {}", &args[1]),
        }

        let mut file = File::open(&args[2]).expect(&format!("Failed to open file {}", &args[2]));
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");

        let json_data: Value = serde_json::from_str(&contents).expect("Failed to parse JSON");

        let args: Vec<String> = env::args().collect();
        let quoted_args: Vec<String> = args.iter().map(|arg| format!("\"{}\"", arg)).collect();

        set_install_value("CMD", &quoted_args.join(" "));

        render(&json_data, &action)?;

        Ok(())
    } else {
        panic!("No arguments passed");
    }
}
