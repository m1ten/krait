use crate::{self as wix, args::Arguments, exit, question, WixPy};
use std::{fs, process::Command, vec};

pub fn run(_wix_py: WixPy, _args: Arguments) {
    // TODO: Implement setup.rs

    if !question!("All pervious wix data will erased, continue?") {
        exit!(1);
    }

    // remove old wix data
    println!("\nRemoving old wix data...");
    fs::remove_dir_all(dirs::home_dir().unwrap().join("wix")).unwrap_or(());

    // create new wix data
    println!("Creating new wix data...");
    let folder: Vec<&str> = vec!["bin", "cache"];
    for f in folder {
        fs::create_dir_all(dirs::home_dir().unwrap().join("wix/{}".replace("{}", f))).unwrap()
    }

    // create wix.py file
    println!("Creating wix.py file...");


    // let _ = writefs(
    //     dirs::home_dir()
    //         .unwrap()
    //         .join("wix/wix.py")
    //         .to_str()
    //         .unwrap()
    //         .to_string(),
    //     contents,
    // );
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
