use std::{fs, path::Path};

pub mod patch;
mod patches;

fn main() {
    let fpath = match std::env::args().nth(1) {
        Some(s) => s,
        None => {
            println!("usage: {:?} path/to/bhd.exe", std::env::current_exe().unwrap());
            return
        }
    };

    // yeah idc, make it more efficient if you want
    let mut stream = fs::read(&fpath).unwrap();

    // backup existing file if backup doesnt exist already
    let backup_fpath = format!("{fpath}.bak");
    if !Path::new(&backup_fpath).exists() {
        fs::copy(&fpath, backup_fpath).unwrap();
    } else {
        println!("skipped making backup of executable because a backup file was already found")
    }

    if fpath.ends_with("bhd.exe") {
        for patch in patches::REHD_PATCHES {
            if let Err(why) = patch.apply(&mut stream) {
                println!("Error during patching: {:?}", why);
            }
        }
    }
    else {
        todo!("re0 is not supported atm")
    }

    fs::write(&fpath, &stream).unwrap();

    println!("done patching and writing");
}
