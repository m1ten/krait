use crate::{self as neo, args::Args, exit, question, NeoConfig};
use std::{fs, path::PathBuf, vec};

pub fn run(path: PathBuf, neo_config: NeoConfig, _args: Args) {
    // TODO: Implement setup.rs

    // struct to toml
    let config_toml =
        toml::to_string(&neo_config).expect("Error: Could not convert neo config to toml");

    bat::PrettyPrinter::new()
        .input_from_bytes(config_toml.as_bytes())
        .language("toml")
        .line_numbers(true)
        .grid(true)
        .theme("Visual Studio Dark+")
        .print()
        .expect("Error: Could not print toml");

    if !question!("All previous neo data will be erased, continue?") {
        exit!(1);
    }

    // remove old neo data
    println!("\nRemoving old neo data...");
    match fs::remove_dir_all(&path) {
        Ok(_) => println!("Old neo data removed..."),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("No old neo data found...");
            } else {
                eprintln!("\nError removing old neo data: {}", e);
                exit!(1);
            }
        }
    }

    // create new neo folders
    println!("Creating new neo folders...");
    let folder: Vec<&str> = vec!["bin", "cache"];
    for f in folder {
        fs::create_dir_all(path.clone().join(f)).unwrap()
    }

    // create config.neo file
    println!("Creating config.neo file...");
    let _ = neo::writefs(
        match path.clone().join("config.neo").to_str() {
            Some(x) => x.to_string(),
            None => {
                eprintln!("Error: Creating config.neo file.");
                exit!(1);
            }
        },
        config_toml,
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
