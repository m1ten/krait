use std::{process::Command, vec};

use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    println!("works");
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

// function to check if python is installed
pub fn is_python_installed() -> bool {
    let name: Vec<&str> = vec!["py","python", "python3", "pypy", "pypy3"];
    let version: Vec<&str> = vec!["3.8", "3.9", "3.10"];
    for i in name.iter() {
        for j in version.iter() {
            let output = match Command::new(*i).arg("--version").output() {
                Ok(o) => o,
                Err(_) => return false
            };
            let output = String::from_utf8_lossy(&output.stdout).to_string();
            if output.contains(j) {
                return true;
            }
        }
    }
    false
}
