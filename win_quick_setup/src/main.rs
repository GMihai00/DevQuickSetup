mod commands;
mod executor_factory;
mod rendering;

use log::{error, info, warn, LevelFilter};
use simplelog::{Config, TermLogger, TerminalMode};
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::exit;

use commands::common::set_install_value;
use commands::common::InstallActionType;
use rendering::install_config;

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

        match save_cmd(args.clone()) {
            Err(err) => {
                warn!(
                    "Failed to save cmd, this might affect installation, err: {}",
                    err
                );
            }
            _ => {}
        }

        match install_config(&args[2].clone(), &action).await {
            Ok(ret) => {
                if !ret {
                    error!("One of the commands durring instalation failed, halting execution");
                    exit(5);
                }
            }
            Err(err) => {
                error!("Failed to install, err: {}", err);
                exit(5);
            }
        }

        // time elapsed here could be nice
        info!("Instalation finished");
        exit(0);
    }

    error!("Invalid arguments passed! Valid cmd example: \"win_quick_setup --install Conf.json\"");
    exit(2);
}
