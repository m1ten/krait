use crate::{self as neopkg, args::Args, exit, question, NeoConfig};
use std::{fs, path::PathBuf, vec};

pub fn run(path: PathBuf, neo_config: NeoConfig, _args: Args) {
    // TODO: Implement setup.rs

    // struct to yaml
    let config_yaml =
        serde_yaml::to_string(&neo_config).expect("Error: Could not convert neopkg config to yaml.");

    bat::PrettyPrinter::new()
        .input_from_bytes(config_yaml.as_bytes())
        .language("yaml")
        .line_numbers(true)
        .grid(true)
        .theme("Visual Studio Dark+")
        .print()
        .expect("Error: Could not print yaml.");

    if !question!("All previous neopkg data will be erased, continue?") {
        exit!(1);
    }

    // remove old neopkg data
    println!("\nRemoving old neopkg data...");
    match fs::remove_dir_all(&path) {
        Ok(_) => println!("Old neopkg data removed..."),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("No old neopkg data found...");
            } else {
                eprintln!("\nError removing old neopkg data: {}", e);
                exit!(1);
            }
        }
    }

    // create new neopkg folders
    println!("Creating new neopkg folders...");
    let folder: Vec<&str> = vec!["bin", "cache"];
    for f in folder {
        fs::create_dir_all(path.clone().join(f)).unwrap()
    }

    // create config.neopkg file
    println!("Creating neopkg.yml file...");
    let _ = neopkg::writefs(
        match path.clone().join("neopkg.yml").to_str() {
            Some(x) => x.to_string(),
            None => {
                eprintln!("Error: Creating neopkg.yml file.");
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
