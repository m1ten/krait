use crate::{self as wix, args::Arguments, exit, question, WixConfig};
use std::{fs, process::Command, vec, path::PathBuf};

pub fn run(path: PathBuf, _wix_config: WixConfig, _args: Arguments) {
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

    // create wix.py file
    println!("Creating wix.py file...");
    let _ = wix::writefs(match path.clone().join("wix.py").to_str() {
        Some(x) => x.to_string(),
        None => {
            eprintln!("Error: Creating wix.py file.");
            exit!(1);
        }
    }, todo!());
}

pub fn venv(venv_path: PathBuf) -> bool {
    let name: Vec<&str> = vec!["py", "python3", "python"];

    for i in name {
        let venv = Command::new(i)
            .arg("-m")
            .arg("venv")
            .arg(venv_path.clone())
            .output()
            .expect("Failed to create virtual environment");

        if venv.status.success() {
            println!("Virtual environment created!");
            return true;
        }
    }

    false
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
pub fn is_python_installed(v: &str) -> bool {
    let name: Vec<&str> = vec!["py", "python3", "python"];
    for n in name.iter() {
        match Command::new(n).arg("--version").output() {
            Ok(o) => {
                if String::from_utf8(o.stdout).unwrap().contains(v) {
                    return true;
                }
            }
            Err(_) => continue,
        };
    }
    false
}

// check if there is a internet connection
pub async fn is_internet_connected() -> bool {
    online::check(None).await.is_ok()
}

// check if wix is in a venv
pub fn is_venv() -> bool {
    match std::env::var("VIRTUAL_ENV") {
        Ok(_) => true,
        Err(_) => false,
    }
}

// names are not finalized yet
// os/arch not in the list will be ignored

// get the current operating system
pub fn get_os() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "freebsd") {
        "freebsd".to_string()
    } else if cfg!(target_os = "dragonfly") {
        "dragonfly".to_string()
    } else if cfg!(target_os = "openbsd") {
        "openbsd".to_string()
    } else if cfg!(target_os = "netbsd") {
        "netbsd".to_string()
    } else if cfg!(target_os = "ios") {
        "ios".to_string()
    } else if cfg!(target_os = "android") {
        "android".to_string()
    } else {
        "unknown".to_string()
    }
}

// get the current architecture
pub fn get_arch() -> String {
    if cfg!(target_arch = "x86_64") {
        "x86_64".to_string()
    } else if cfg!(target_arch = "x86") {
        "x86".to_string()
    } else if cfg!(target_arch = "arm") {
        "arm".to_string()
    } else if cfg!(target_arch = "aarch64") {
        "aarch64".to_string()
    } else if cfg!(target_arch = "mips") {
        "mips".to_string()
    } else if cfg!(target_arch = "mipsel") {
        "mipsel".to_string()
    } else if cfg!(target_arch = "powerpc") {
        "powerpc".to_string()
    } else if cfg!(target_arch = "powerpc64") {
        "powerpc64".to_string()
    } else if cfg!(target_arch = "s390x") {
        "s390x".to_string()
    } else if cfg!(target_arch = "sparc64") {
        "sparc64".to_string()
    } else if cfg!(target_arch = "sparc") {
        "sparc".to_string()
    } else {
        "unknown".to_string()
    }
}
