use serde::{Serialize, Deserialize};
use clap::{App, Arg};
use std::fs;
use std::io::{Read, Write};

#[derive(std::fmt::Debug, Clone, Serialize, Deserialize)]
pub struct Variables {
	pub name: String,
	pub version: String,
	pub author: String,
	pub description: String,
}

pub struct Arguments {
	pub confirm: bool,
	pub verbose: bool,
}

impl Arguments {
	pub fn run(vars: Variables) -> Arguments {
		// create custom args
		let matches = App::new(vars.name)
			.version(vars.version.as_str())
			.author(vars.author.as_str())
			.about(vars.description.as_str())
			.arg(
				Arg::with_name("no confirm")
					.short("y")
					.long("noconfirm")
					.value_name("NOCONFIRM")
					.help("yes to everything")
					.takes_value(false),
			)
			.arg(
				Arg::with_name("verbose")
					.short("v")
					.long("verbose")
					.value_name("Verbose")
					.help("print logs")
					.takes_value(false),
			)
			.get_matches();
		// return custom args
		let mut data: Vec<bool> = Vec::new();

		match matches.occurrences_of("no confirm") {
			1 => data.push(true),
			_ => data.push(false),
		}
		match matches.occurrences_of("verbose") {
			1 => data.push(true),
			_ => data.push(false),
		}

		return Arguments {
			confirm: data[0],
			verbose: data[1],
		};
	}
}

#[derive(std::fmt::Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub info: Option<Variables>,
	pub setup: Option<Setup>
}

impl Config {
	pub fn read() -> Config {
		let mut file = fs::OpenOptions::new().read(true).open("dash.toml").unwrap();
		let mut contents = String::new();
    	match file.read_to_string(&mut contents) {
        	Ok(_) => (),
        	Err(_) => println!("error reading from file: {}.", "dash.toml"),
    	}

		return toml::from_str(contents.as_str()).unwrap(); 
	}

	pub fn write(contents: Config) {
		let mut file = fs::OpenOptions::new()
        	.read(true)
        	.write(true)
        	.create(true)
        	.open("dash.toml")
        	.unwrap();

		println!("{:?}", contents);

    	// convert Config to toml::Value to String
    	let toml_value = toml::Value::try_from(contents).unwrap();
		let toml_string = toml::to_string_pretty(&toml_value).unwrap();

    	// convert string to byte & write to file
    	match file.write_all(toml_string.as_bytes()) {
        	Ok(_) => (),
        	Err(_) => println!("error writing to file: {}.", "dash.toml"),
    	}
	}
}

#[derive(std::fmt::Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
	pub os: String,
	pub distro: Option<String>,
	pub pkg_mgr: Option<String>,
	pub pkg: Vec<Pkgfile>,
	pub dotfile: Vec<Pkgfile>,
}

#[derive(std::fmt::Debug, Clone, Serialize, Deserialize)]
pub struct Pkgfile {
	pub name: String,
	pub prv_path: Option<String>,
	pub new_path: Option<String>
}