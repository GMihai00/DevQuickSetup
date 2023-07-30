use super::commands;

use commands::common::ActionFn;
use commands::conditional_command::ConditionalCommandExecutor;
use commands::delete_reg_key_command::DeleteRegistryValueCommandExecutor;
use commands::dir_command::DirCommandExecutor;
use commands::exec_command::ExecCommandExecutor;
use commands::get_reg_value_command::GetRegistryValueCommandExecutor;
use commands::include_command::IncludeCommandExecutor;
use commands::paralel_exec_command::ParalelExecCommandExecutor;
use commands::ps1_command::PowershellCommandExecutor;
use commands::set_reg_value_command::UpdateRegistryCommandExecutor;
use commands::set_var_command::SetVarCommandExecutor;
use commands::vcpkg_command::VcpkgCommandExecutor;
use commands::winget_command::WingetCommandExecutor;

pub struct ExecutorFactory {}

impl ExecutorFactory {
    pub fn build(name: &str) -> Box<dyn ActionFn + Send + Sync> {
        match name {
            "exec" => {
                return Box::new(ExecCommandExecutor {});
            }
            "winget" => {
                return Box::new(WingetCommandExecutor {});
            }
            "include" => {
                return Box::new(IncludeCommandExecutor {});
            }
            "reg_update" => return Box::new(UpdateRegistryCommandExecutor {}),
            "ps1" => return Box::new(PowershellCommandExecutor {}),
            "vcpkg" => return Box::new(VcpkgCommandExecutor {}),
            "dir" => return Box::new(DirCommandExecutor {}),
            "set_reg_val" => return Box::new(UpdateRegistryCommandExecutor {}),
            "set_var" => return Box::new(SetVarCommandExecutor {}),
            "get_reg_val" => return Box::new(GetRegistryValueCommandExecutor {}),
            "if" => return Box::new(ConditionalCommandExecutor {}),
            "delete_reg_key" => return Box::new(DeleteRegistryValueCommandExecutor {}),
            "paralel" => return Box::new(ParalelExecCommandExecutor {}),
            _ => {
                // OK to panic here
                panic!("Failed to create command corresponding to name: {}", name)
            }
        }
    }
}
