use serde_json::Value;
use std::error::Error;
pub enum InstallActionType {
    INSTALL,
    UNINSTALL,
    UPDATE,
}

pub type ActionFn = fn(&Value, &InstallActionType) -> Result<bool, Box<dyn Error>>;