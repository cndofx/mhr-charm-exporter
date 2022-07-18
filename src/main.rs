mod constants;
mod data;
mod datamanager;
mod process;

use datamanager::DataManager;
use std::fs::File;
use std::io::{BufWriter, Write};

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
