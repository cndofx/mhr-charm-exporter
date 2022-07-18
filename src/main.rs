mod constants;
mod structs;
mod process;
mod data;

use std::fs::File;
use std::io::{BufWriter, Write};
use process_memory::{DataMember, Memory, ProcessHandle};
use data::DataManager;

#[cfg(windows)]
fn main() {
    

    let proc = process::get_process_handle("MonsterHunterRise.exe").unwrap();

    let manager = DataManager::new(proc);

    let charms = manager.get_all_charms();

    let file = File::create("charms.txt").unwrap();
    let mut writer = BufWriter::new(&file);

    for c in charms {
        writeln!(&mut writer, "{}", c).unwrap();
    }
}