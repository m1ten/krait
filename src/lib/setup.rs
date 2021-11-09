use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    // rewrite these functions because they suck and won't work on linux.
    if is_super() {
        eprintln!("{}", "Error: You are running wix as root.");
        std::process::exit(1);
    }

    if !is_python_installed() {
        eprintln!("Error: Python is not installed.");
        std::process::exit(1);
    }

    println!("works");
}

// function to check if running as root/admin
fn is_super() -> bool {
    #[cfg(windows)]
    {
        let ps = match which::which("powershell") {
            Ok(path) => path,
            Err(_) => {
                eprintln!("Error: powershell is not installed.");
                std::process::exit(1);
            }
        };
        
        let output = std::process::Command::new(ps)
            .arg("-Command")
            .arg("Get-WmiObject -Class Win32_UserAccount -Filter \"Name='Administrator'\" -Property LocalAccount")
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.contains("True")
    }

    #[cfg(not(windows))]
    {
        nix::unistd::getuid().is_root()
    }
}

// function to check if python is installed
fn is_python_installed() -> bool {
    match which::which("python3") {
        Ok(_) => true,
        Err(_) => false,
    }
}
