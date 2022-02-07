use crate::{self as wix, args::Args, exit, question, WixConfig};
use std::{fs, vec, path::PathBuf};

pub fn run(path: PathBuf, _wix_config: WixConfig, _args: Args) {
    // TODO: Implement setup.rs

    if !question!("All pervious wix data will erased, continue?") {
        exit!(1);
    }

    // remove old wix data
    println!("\nRemoving old wix data...");
    match fs::remove_dir_all(&path) {
        Ok(_) => println!("\nOld wix data removed..."),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("\nNo old wix data found...");
            } else {
                eprintln!("\nError removing old wix data: {}", e);
                exit!(1);
            }
        }
    }

    // create new wix folders
    println!("Creating new wix folders...");
    let folder: Vec<&str> = vec!["bin", "cache"];
    for f in folder {
        fs::create_dir_all(path.clone().join(f)).unwrap()
    }



    // create config.wix file
    println!("Creating config.wix file...");
    let _ = wix::writefs(match path.clone().join("config.wix").to_str() {
        Some(x) => x.to_string(),
        None => {
            eprintln!("Error: Creating config.wix file.");
            exit!(1);
        }
    }, todo!());
}

// function to check if running as root/admin
pub fn is_super() -> bool {
    #[cfg(windows)]
    {
        is_elevated::is_elevated()
    }

    #[cfg(not(windows))]
    {
        nix::unistd::getuid().is_root()
    }
}

// check if there is a internet connection
pub async fn is_internet_connected() -> bool {
    online::check(None).await.is_ok()
}

// get the current operating system
pub fn get_os() -> String {
    std::env::consts::OS.to_string()
}

// get the current architecture
pub fn get_arch() -> String {
    std::env::consts::ARCH.to_string()
}