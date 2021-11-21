#[path = "lang/py.rs"]
pub mod lang;

#[path = "lib/setup.rs"]
pub mod setup;

#[path = "lib/args.rs"]
pub mod args;

#[path = "lib/structs.rs"]
pub mod structs;

use std::{fs::File, io::{self, Read, Write}};

// read from file
pub fn readfs(path: String) -> Result<String, io::Error> {
	let mut file = File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}

// write to file
pub fn writefs(path: String, contents: String) -> Result<(), io::Error> {
	let mut file = File::create(path)?;
	file.write_all(contents.as_bytes())?;
	Ok(())
}

// read from stdin
pub fn scan<T: std::str::FromStr>(stopper: u8) -> Result<T, ()> {
    let mut input = Vec::<u8>::new();

    let mut data: [u8; 1] = [0];
    loop {
        match std::io::stdin().read_exact(&mut data) {
            Ok(_) => {}
            Err(_) => return Err(()),
        }

        if data[0] != stopper && data[0] != '\n' as u8 {
            input.push(data[0]);
        } else {
            break;
        }
    }

    match std::str::from_utf8(&input).unwrap().trim().parse::<T>() {
        Ok(x) => Ok(x),
        Err(_) => Err(()),
    }
}

#[macro_export]
macro_rules! scan {
    ($str:tt, $_type:ty) => {{
		print!("{}", $str);
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
        wix::scan::<$_type>(' ' as u8).expect("scan failed")
    }};
}

#[macro_export]
macro_rules! scanln {
	($str:tt) => {{
		print!("{}", $str);
		std::io::Write::flush(&mut std::io::stdout()).unwrap();
		wix::scan::<String>('\n' as u8).expect("scanln failed")
	}};
}

// macro to clear console
#[macro_export]
macro_rules! clear {
	() => {{
		// if cfg!(target_os = "windows") {
		// 	print!("\x1B[2J");
		// } else {
		// 	print!("\x1B[2J\x1B[1;1H");
		// }
		// std::io::Write::flush(&mut std::io::stdout()).unwrap();

		use std::process::Command;

		if cfg!(target_os = "windows") {
			Command::new("cmd").arg("/c").arg("cls").status().unwrap();
		} else {
			Command::new("clear").status().unwrap();
		}

	}};
}

#[macro_export]
macro_rules! exit {
	($code: tt) => {{
		let key = if cfg!(target_os = "macos") { "return" } else { "enter" };
		let msg = format!("\nPress {} to exit.\n", key);
		wix::scanln!(msg);
    	std::process::exit($code);
	}};
}

#[macro_export]
macro_rules! question {
	($msg: tt) => {{
		loop {
			print!("{} [Y/n] ", $msg);
			std::io::Write::flush(&mut std::io::stdout()).unwrap();
			let answer = wix::scan::<String>('\n' as u8).expect("question failed").to_lowercase();
			if answer.trim() == "y" || answer.trim() == "yes" || answer.trim() == "" {
				break true;
			} else if answer.trim() == "n" || answer.trim() == "no" {
				break false;
			}
		}
	}}
}