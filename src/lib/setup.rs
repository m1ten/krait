use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
    // TODO: Implement setup.rs

    is_super();

    if is_python_installed() {
        let interpreter: String = loop {
            println!("Python was not found, install it?");
            let i = wix::scanln!("Press 'enter' to exit. [Python: 1, PyPy: 2]: ");

            if i == "".to_string() {
                return;
            } else {
                match i.parse::<u8>() {
                    Ok(1) => break "Python".to_string(),
                    Ok(2) => break "PyPy".to_string(),
                    _ => wix::clear!(),
                }
            }
        };

        install_python(interpreter);
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
    let output = std::process::Command::new("lsb_release").arg("-a").output();

    match output {
        Ok(o) if o.status.success() => println!("{:?}", o.stdout),
        Ok(o) => panic!("{}", o.status),
        Err(_) => panic!("Failed to get linux distro."),
    }
}

fn install_python(interpreter: String) {
	let url;

	if cfg!(target_os = "linux") {
		url = match interpreter.as_str() {
			"Python" => "https://www.python.org/ftp/python/3.10.0/Python-3.10.0.tar.xz",
			"PyPy" => "https://downloads.python.org/pypy/pypy3.8-v7.3.7-linux64.tar.bz2",
			_ => panic!("Unsupported interpreter."),
		};
	} else if cfg!(target_os = "windows") {
		url = match interpreter.as_str() {
			"Python" => "https://www.python.org/ftp/python/3.10.0/python-3.10.0-amd64.exe",
			"PyPy" => "https://downloads.python.org/pypy/pypy3.8-v7.3.7-win64.zip",
			_ => panic!("Unsupported interpreter."),
		};
	} else if cfg!(target_os = "macos") {
		url = match interpreter.as_str() {
			"Python" => "https://www.python.org/ftp/python/3.10.0/python-3.10.0post2-macos11.pkg",
			"PyPy" => "https://downloads.python.org/pypy/pypy3.8-v7.3.7-osx64.tar.bz2",
			_ => panic!("Unsupported interpreter."),
		};
	} else {
		panic!("Unsupported platform.");
	}

	println!("{}", url);
}