use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    println!("works");
}

// function to check if running as root/admin
pub fn is_super() -> bool {
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
pub fn is_python_installed() -> bool {
    match which::which("python") {
        Ok(_) => true,
        Err(_) => false,
    }
}
