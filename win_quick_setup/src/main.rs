mod commands;
mod executor_factory;
mod rendering;

use log::{error, info, warn, LevelFilter};
use serde_json::Value;
use simplelog::{Config, TermLogger, TerminalMode};
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use commands::common::set_install_value;
use commands::common::InstallActionType;
use rendering::render;

fn save_cmd(mut args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
    match fs::canonicalize(args[2].clone().as_str()) {
        Ok(config_absolute_path) => {
            args[2] = config_absolute_path.to_string_lossy().to_string();

            let conf_file = args[2].clone();

            let conf_dir = Path::new(conf_file.as_str()).parent();
            if conf_dir != None {
                let conf_dir = conf_dir.unwrap();
                let conf_dir = conf_dir.to_str().unwrap().to_owned() + "\\";
                set_install_value("CONF_DIR", conf_dir);

                let quoted_args: Vec<String> =
                    args.iter().map(|arg| format!("\"{}\"", arg)).collect();

                set_install_value("CMD", &quoted_args.join(" "));

                return Ok(());
            } else {
                return Err(format!(
                    "Failed to get parent directory of config file \"{}\"",
                    conf_file
                )
                .into());
            }
        }
        Err(err) => {
            return Err(format!(
                "Failed to get absolute path for file: \"{}\", err: {}",
                args[2], err
            )
            .into());
        }
    }
}

fn load_config_file(args: &Vec<String>) -> Result<Value, Box<dyn Error + Send + Sync>> {
    match File::open(&args[2]) {
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
                        format!("Failed to read file: \"{}\" err: {}", &args[2], err).into(),
                    );
                }
            }
        }
        Err(err) => {
            return Err(format!("Failed to open file: \"{}\", err: {}", &args[2], err).into());
        }
    }
}

#[tokio::main]
async fn main() {
    match TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to init loger err: {}", err);
            exit(1);
        }
    }

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let action: InstallActionType;
        match &args[1].as_str() {
            &"--install" => action = InstallActionType::INSTALL,
            &"--uninstall" => action = InstallActionType::UNINSTALL,
            &"--update" => action = InstallActionType::UPDATE,
            _ => {
                error!("Invalid option passed {}", &args[1]);
                // std::io::ErrorKind::Unsoported; to refactor all of this exit codes
                exit(36);
            }
        }

        match load_config_file(&args) {
            Ok(json_data) => {
                match save_cmd(args) {
                    Err(err) => {
                        warn!(
                            "Failed to save cmd, this might affect installation, err: {}",
                            err
                        );
                    }
                    _ => {}
                }

                match render(&json_data, &action).await {
                    Err(err) => {
                        error!("Failed to install, err: {}", err);
                        exit(5);
                    }
                    _ => {
                        info!("Instalation finished");
                    }
                }
            }
            Err(err) => {
                error!("Failed to load config err: {}", err);
                exit(5);
            }
        }
    } else {
        error!("No arguments passed");
        exit(2);
    }
}
