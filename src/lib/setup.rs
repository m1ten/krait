use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    // rewrite these functions because they suck and won't work on linux.
    is_super();
    is_python_installed();
}

// function to check if running as root/admin
fn is_super() -> bool {
}

// function to check if python is installed
fn is_python_installed() -> bool {
}

// function to get linux distro
#[cfg(target_os = "linux")]
fn get_linux_distro() {
}