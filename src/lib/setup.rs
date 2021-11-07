use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    is_super();

	if is_python_installed() {
		let install: String = loop {
			println!("Python was not found, install it?");
			let i = wix::scanln!("[Python: 1, PyPy: 2]: ");

			if i == "".to_string() {
				return;
			} else {
				match i.parse::<u8>() {
					Ok(1) => break "Python".to_string(),
					Ok(2) => break "PyPy".to_string(),
					_ => wix::clear!()
				}
			}
		};

		println!("Installing {}.", install);
	}
}

// function to check if running as root/admin
fn is_super() -> bool {
    if cfg!(unix) {
        match std::env::var("USER") {
            Ok(user) => match user.as_str() {
                "root" => panic!("Please run wix as a regular user."),
                _ => false,
            },
            Err(e) => panic!("{}", e),
        }
    } else if cfg!(windows) {
        match std::env::var("USERNAME") {
            Ok(user) => match user.as_str() {
                "Administrator" => panic!("Please run wix as a regular user."),
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
            Ok(o) => panic!("{}", o.status),
            Err(_) => false,
        }
    } else if cfg!(windows) {
        let output = std::process::Command::new("python")
            .arg("--version")
            .output();

        match output {
            Ok(o) if o.status.success() => true,
            Ok(o) => panic!("{}", o.status),
            Err(_) => false,
        }
    } else {
        panic!("Unsupported platform.");
    }
}

// function to get linux distro
#[cfg(target_os = "linux")]
fn get_linux_distro() {
	if cfg!(unix) {
		let output = std::process::Command::new("lsb_release")
			.arg("-a")
			.output();

		match output {
			Ok(o) if o.status.success() => println!("{:?}", o.stdout),
			Ok(o) => panic!("{}", o.status),
			Err(_) => panic!("Failed to get linux distro."),
		}
	} else {
		panic!("Unsupported platform.");
	}
}