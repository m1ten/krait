use pyo3::ffi::printfunc;

use crate as wix;

pub fn run(info: wix::structs::Information, args: wix::args::Arguments) {
	// TODO: Implement setup.rs

	// macro to run the following code if args.assume_yes is false
	macro_rules! no_perm {
		($code:expr) => {
			if !args.assume_yes {
				$code
			}
		};
	}

	is_super();

	no_perm!({
		println!("assume_yes is false")
	});

}

// function to check if running as root/admin
fn is_super() -> bool {
	if cfg!(unix) {
    	match std::env::var("USER") {
        	Ok(user) => match user.as_str() {
            	"root" => panic!("Please run wix as a regular user."),
            	_ => false
        	},
        	Err(e) => panic!("{}", e)
    	}
	} else if cfg!(windows) {
		match std::env::var("USERNAME") {
			Ok(user) => match user.as_str() {
				"Administrator" => panic!("Please run wix as a regular user."),
				_ => false
			},
			Err(e) => panic!("{}", e)
		}
	} else {
		panic!("Unsupported platform.");
	}
}