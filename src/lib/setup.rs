use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    println!("works");
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
pub fn is_python_installed() -> bool {
    match which::which("python") {
        Ok(_) => true,
        Err(_) => false,
    }
}
