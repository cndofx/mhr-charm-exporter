mod constants;
mod data;
mod datamanager;
mod process;

use datamanager::DataManager;
use std::fs::File;
use std::io::{BufWriter, Write};

#[cfg(windows)]
fn main() -> anyhow::Result<()> {
    let proc = process::get_process_handle("MonsterHunterRise.exe")?;

    let manager = DataManager::new(proc);

    let charms = manager.get_all_charms();

    let file = File::create("charms.txt")?;
    let mut writer = BufWriter::new(&file);

    for c in charms {
        writeln!(&mut writer, "{}", c)?;
        println!("{}", c);
    }

    println!("\ncharms saved to charms.txt");
    Ok(())
}
