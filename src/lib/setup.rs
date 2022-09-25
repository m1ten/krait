use crate::{self as krait, exit, question, KraitConfig};
use std::fs;

use console::{Emoji, style};

pub fn run(krait_config: KraitConfig) {
    // TODO: Implement setup.rs

    let krait_path = krait_config.dir.clone();
    let krait_path_lua = krait_path.join("krait.lua");

    let config_string = krait_config.gen_lua();


    // TODO: add support for text editors (vim, nano, etc)
    bat::PrettyPrinter::new()
        .input_from_bytes(config_string.as_bytes())
        .language("lua")
        .line_numbers(true)
        .grid(true)
        .theme("Visual Studio Dark+")
        .print()
        .expect("Error: Could not print yaml.");

    if !question!("All previous krait data will be erased, continue?") {
        exit!(1);
    }

    // remove old krait data
    // println!("\nRemoving old krait data...");

    println!(
        "{} {}Removing old krait data...",
        style("[1/4]").bold().dim(),
        Emoji("🗑️  ", "")
    );

    match fs::remove_dir_all(&krait_path) {
        Ok(_) => println!(
            "{} {}Old krait data removed...",
            style("[2/4]").bold().dim(),
            Emoji("🚫  ", "")
        ),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!(
                    "{} {}No previous krait data found...",
                    style("[2/4]").bold().dim(),
                    Emoji("🚫  ", "")
                );
            } else {
                eprintln!(
                    "{} {}Error removing old krait data: {}",
                    style("[2/4]").bold().dim(),
                    Emoji("🔥  ", ""),
                    e
                );
                exit!(1);
            }
        }
    }

    println!(
        "{} {}Creating new krait config...",
        style("[3/4]").bold().dim(),
        Emoji("📝  ", "")
    );

    // create new krait folders
    // println!("Creating new krait folders...");
    for f in &["pkg", "cache"] {
        fs::create_dir_all(krait_path.clone().join(f)).unwrap()
    }

    // create krait.lua file
    // println!("Creating krait.lua file...");
    let _ = krait::writefs(
        match krait_path_lua.to_str() {
            Some(x) => x.to_string(),
            None => {
                eprintln!("Error: Creating krait.lua file.");
                exit!(1);
            }
        },
        config_string,
    );

    // println!("\nSetup complete!");

    println!(
        "{} {}Setup Complete...",
        style("[4/4]").bold().dim(),
        Emoji("⚙️  ", "")
    );
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
    online::check(None).is_ok()
}

// get the current operating system
pub fn get_os() -> String {
    std::env::consts::OS.to_string()
}

// get the current architecture
pub fn get_arch() -> String {
    std::env::consts::ARCH.to_string()
}
