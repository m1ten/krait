use crate::{self as wix, exit, question, WixConfig};
use std::fs;

pub fn run(wix_config: WixConfig) {
    // TODO: Implement setup.rs

    let wix_path = wix_config.dir.clone();
    let wix_path_yml = wix_path.join("wix.yml");

    // struct to yaml
    let config_yaml =
        serde_yaml::to_string(&wix_config).expect("Error: Could not convert wix config to yaml.");

    bat::PrettyPrinter::new()
        .input_from_bytes(config_yaml.as_bytes())
        .language("yaml")
        .line_numbers(true)
        .grid(true)
        .theme("Visual Studio Dark+")
        .print()
        .expect("Error: Could not print yaml.");

    if !question!("All previous wix data will be erased, continue?") {
        exit!(1);
    }

    // remove old wix data
    println!("\nRemoving old wix data...");
    match fs::remove_dir_all(&wix_path) {
        Ok(_) => println!("Old wix data removed..."),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("No old wix data found...");
            } else {
                eprintln!("\nError removing old wix data: {}", e);
                exit!(1);
            }
        }
    }

    // create new wix folders
    println!("Creating new wix folders...");
    for f in &["pkg", "cache"] {
        fs::create_dir_all(wix_path.clone().join(f)).unwrap()
    }

    // create wix.yml file
    println!("Creating wix.yml file...");
    let _ = wix::writefs(
        match wix_path_yml.to_str() {
            Some(x) => x.to_string(),
            None => {
                eprintln!("Error: Creating wix.yml file.");
                exit!(1);
            }
        },
        config_yaml,
    );

    println!("\nSetup complete!");
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
