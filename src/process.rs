use process_memory::{ProcessHandle, TryIntoProcessHandle};
use sysinfo::{System, SystemExt, ProcessExt, AsU32};

#[derive(Debug)]
pub enum GameOpenError {
    NoGameProcessFound,
    FailedToOpenProcess,
}

pub fn open_game_process() -> Result<ProcessHandle, GameOpenError> {
    let mut sys = System::new_all();
    sys.refresh_components();

    let procs = sys.process_by_name("MonsterHunterRise.exe");

    if procs.is_empty() {
        return Err(GameOpenError::NoGameProcessFound);
    }

    if let Ok(proc) = procs[0].pid().as_u32().try_into_process_handle() {
        Ok(proc)
    } else {
        Err(GameOpenError::FailedToOpenProcess)
    }
}