use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    is_super();
    is_python_installed();
}

// function to check if running as root/admin
fn is_super() -> bool {
    if cfg!(unix) {
        match std::env::var("USER") {
            Ok(user) => match user.as_str() {
                "root" => {
                    eprintln!("Please run wix as a regular user.");
                    std::process::exit(1)
                }
                _ => false,
            },
            Err(e) => panic!("{}", e),
        }
    } else if cfg!(windows) {
        match std::env::var("USERNAME") {
            Ok(user) => match user.as_str() {
                "Administrator" => {
                    eprintln!("Please run wix as a regular user.");
                    std::process::exit(1)
                }
                _ => false,
            },
            Err(e) => panic!("{}", e),
        }
    } else {
        panic!("Unsupported platform.");
    }
}

// function to check if python is installed
fn is_python_installed() -> bool {
    if cfg!(unix) {
        let output = std::process::Command::new("python3")
            .arg("--version")
            .output();

        match output {
            Ok(o) if o.status.success() => true,
            Ok(o) => {
                eprintln!("{}", o.status);
                std::process::exit(1)
            }
            Err(_) => false,
        }
    } else if cfg!(windows) {
        let output = std::process::Command::new("python")
            .arg("--version")
            .output();

        match output {
            Ok(o) if o.status.success() => true,
            Ok(o) => {
                eprintln!("{}", o.status);
                std::process::exit(1)
            }
            Err(_) => false,
        }
    } else {
        panic!("Unsupported platform.");
    }
}

// function to get linux distro
#[cfg(target_os = "linux")]
fn get_linux_distro() {
    let output = std::process::Command::new("lsb_release").arg("-a").output();

    match output {
        Ok(o) if o.status.success() => println!("{:?}", o.stdout),
        Ok(o) => panic!("{}", o.status),
        Err(_) => panic!("Failed to get linux distro."),
    }
}
