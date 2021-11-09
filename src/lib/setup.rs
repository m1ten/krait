use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    // rewrite these functions because they suck and won't work on linux.
    if is_super() {
        panic!("You are not allowed to run this program as root.");
    }
    if !is_python_installed() {
        panic!("Python is not installed.");
    }
}

// function to check if running as root/admin
fn is_super() -> bool {
    #[cfg(windows)]
    {
        let output = std::process::Command::new("powershell")
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
    if which::which("python3").is_ok() || which::which("python").is_ok() {
        return true;
    } else {
        return false;
    }
}
