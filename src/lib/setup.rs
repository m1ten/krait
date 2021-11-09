use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    // rewrite these functions because they suck and won't work on linux.
    is_super();
    is_python_installed();
}

// function to check if running as root/admin
fn is_super() -> bool {
    if cfg!(target_os = "macos") {
        let output = std::process::Command::new("id")
            .arg("-g")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        if output.trim() == "0" {
            return true;
        } else {
            return false;
        }

    } else if cfg!(target_os = "linux") {
        let output = std::process::Command::new("id")
            .arg("-g")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        if output.trim() == "0" {
            return true;
        } else {
            return false;
        }

    } else if cfg!(target_os = "windows") {
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg("Get-WmiObject -Class Win32_UserAccount -Filter \"LocalAccount=TRUE\" -Property Name")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        if output.trim() == "Administrator" {
            return true;
        } else {
            return false;
        }

    } else {
        panic!("Unsupported OS");
    }
}

// function to check if python is installed
fn is_python_installed() -> bool {
    if cfg!(target_os = "macos") {
        let output = std::process::Command::new("which")
            .arg("python3")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        if output.trim() == "/usr/bin/python3" {
            return true;
        } else {
            return false;
        }
    } else if cfg!(target_os = "linux") {
        let output = std::process::Command::new("which")
            .arg("python3")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        if output.trim() == "/usr/bin/python3" {
            return true;
        } else {
            return false;
        }
    } else if cfg!(target_os = "windows") {
        let output = std::process::Command::new("powershell")
            .arg("-Command")
            .arg("Get-WmiObject -Class Win32_Product -Filter \"Name='Python'\" -Property Name")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8_lossy(&output.stdout);
        if output.trim() == "Python" {
            return true;
        } else {
            return false;
        }
    } else {
        panic!("Unsupported OS");
    }
}

// function to get linux distro
#[cfg(target_os = "linux")]
fn get_linux_distro() -> String {
    let output = std::process::Command::new("cat")
        .arg("/etc/os-release")
        .output()
        .expect("failed to execute process");
    let output = String::from_utf8_lossy(&output.stdout);
    output
}