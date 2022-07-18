use process_memory::{ProcessHandle, TryIntoProcessHandle};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessError {
    #[error("game process not found")]
    ProcessNotFound,
    #[error("unable to open game process")]
    ProcessCannotOpen,
}

pub fn get_process_id(process_name: &str) -> Result<u32, ProcessError> {
    let system = System::new_all();
    let mut processes = system.processes_by_name(process_name);

    if let Some(process) = processes.next() {
        Ok(process.pid().as_u32())
    } else {
        Err(ProcessError::ProcessNotFound)
    }
}

pub fn get_process_handle(process_name: &str) -> Result<ProcessHandle, ProcessError> {
    let pid = get_process_id(process_name)?;
    if let Ok(handle) = pid.try_into_process_handle() {
        Ok(handle)
    } else {
        Err(ProcessError::ProcessCannotOpen)
    }
}
