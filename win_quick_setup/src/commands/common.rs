use serde_json::{Value,json};
use std::error::Error;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::option::Option;
use serde::Serialize;
use serde::de::DeserializeOwned;
use regex::Regex;

pub enum InstallActionType {
    INSTALL,
    UNINSTALL,
    UPDATE,
}

pub type ActionFn = fn(&Value, &InstallActionType) -> Result<bool, Box<dyn Error>>;

lazy_static! {
static ref INSTALL_VALUES: Mutex<serde_json::Value> = Mutex::new(json!({}));
}

pub fn get_install_value<T: DeserializeOwned>(key: &str) -> Option<T>{
    
    let install_vals = INSTALL_VALUES.lock().unwrap();
    
    if install_vals.get(key).is_some()
    {
        let val = &install_vals[key];
        
        match serde_json::from_value(val.clone()) {
            Ok(val) => { return Some(val) },
            Err(_err) => { }
        };
    }
    
    return None;
}

pub fn set_install_value<T: Serialize>(key: &str, value: T) {
    let mut install_vals = INSTALL_VALUES.lock().unwrap();
    
    install_vals[key] = json!(value);
}

pub fn expand_string(input_string: &str) -> String {
    
    let re = Regex::new(r"%(.*?)%").unwrap();
    
    let result = re.replace_all(input_string, |caps: &regex::Captures| {
        let captured_value = caps.get(1).map_or("", |m| m.as_str());
        
        let install_val: Option<String> = get_install_value(captured_value);
        match install_val {
            Some(val) => val,
            None => {
                println!("Failed to find install value {}", captured_value);
                return captured_value.to_owned().to_string();
            },
        }
    });
    
    return result.to_string();
}