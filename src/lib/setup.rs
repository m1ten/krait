use crate::{self as krait, exit, question, KraitConfig};
use std::fs;

pub fn run(krait_config: KraitConfig) {
    // TODO: Implement setup.rs

    let krait_path = krait_config.dir.clone();
    let krait_path_yml = krait_path.join("krait.yml");

    // struct to yaml
    let config_yaml =
        serde_yaml::to_string(&krait_config).expect("Error: Could not convert krait config to yaml.");

    bat::PrettyPrinter::new()
        .input_from_bytes(config_yaml.as_bytes())
        .language("yaml")
        .line_numbers(true)
        .grid(true)
        .theme("Visual Studio Dark+")
        .print()
        .expect("Error: Could not print yaml.");

    if !question!("All previous krait data will be erased, continue?") {
        exit!(1);
    }

    // remove old krait data
    println!("\nRemoving old krait data...");
    match fs::remove_dir_all(&krait_path) {
        Ok(_) => println!("Old krait data removed..."),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("No old krait data found...");
            } else {
                eprintln!("\nError removing old krait data: {}", e);
                exit!(1);
            }
        }
    }

    // create new krait folders
    println!("Creating new krait folders...");
    for f in &["pkg", "cache"] {
        fs::create_dir_all(krait_path.clone().join(f)).unwrap()
    }

    // create krait.yml file
    println!("Creating krait.yml file...");
    let _ = krait::writefs(
        match krait_path_yml.to_str() {
            Some(x) => x.to_string(),
            None => {
                eprintln!("Error: Creating krait.yml file.");
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
