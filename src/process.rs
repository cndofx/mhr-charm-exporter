use process_memory::{ProcessHandle, TryIntoProcessHandle};
use sysinfo::{System, SystemExt, ProcessExt, PidExt};

#[derive(Debug)]
pub enum GameOpenError {
    NoGameProcessFound,
    FailedToOpenProcess,
}

pub fn get_process_id(process_name: &str) -> Result<u32, GameOpenError> {
    let system = System::new_all();
    let mut processes = system.processes_by_name(process_name);

    if let Some(process) = processes.next() {
        Ok(process.pid().as_u32())
    } else {
        Err(GameOpenError::NoGameProcessFound)
    }
}

pub fn get_process_handle(process_name: &str) -> Result<ProcessHandle, GameOpenError> {
    let pid = get_process_id(process_name)?;
    if let Ok(handle) = pid.try_into_process_handle() {
        Ok(handle)
    } else {
        Err(GameOpenError::FailedToOpenProcess)
    }
}

// pub fn open_game_process() -> Result<ProcessHandle, GameOpenError> {
//     let mut sys = System::new_all();
//     sys.refresh_components();

//     let procs = sys.processes_by_name("MonsterHunterRise.exe");

//     if procs.is_empty() {
//         return Err(GameOpenError::NoGameProcessFound);
//     }

//     if let Ok(proc) = procs[0].pid().as_u32().try_into_process_handle() {
//         Ok(proc)
//     } else {
//         Err(GameOpenError::FailedToOpenProcess)
//     }
// }

