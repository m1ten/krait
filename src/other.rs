// use std::fmt::Debug;
// use serde::{Serialize, Deserialize};
use clap::{App, Arg};
// use std::fs;
// use std::io::{Read, Write};


// // Basic info about dash
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Variables {
// 	pub name: String,
// 	pub version: String,
// 	pub author: String,
// 	pub description: String,
// }

// // Dash config struct
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Config {
// 	pub info: Option<Variables>,
// 	pub setup: Option<Setup>
// }

// impl Config {
// 	// read toml data from dash.toml -> Config
// 	pub fn read() -> Option<Config> {
// 		let option = fs::OpenOptions::new().read(true).open("dash.toml");
// 		let mut file = if option.is_ok() {
// 			option.unwrap()
// 		} else {
// 			return None;
// 		};

// 		let mut contents = String::new();
//     	match file.read_to_string(&mut contents) {
//         	Ok(_) => (),
//         	Err(e) => println!("error reading from file: {}. Err: {}", "dash.toml", e),
//     	};

// 		// Some(toml::from_str(contents.as_str()).unwrap())
// 		Some(ron::from_str(contents.as_str()).unwrap())
// 	}

// 	// write toml data into dash.toml <- Config
// 	pub fn write(contents: Config) {
// 		let mut file = fs::OpenOptions::new()
//         	.read(true)
//         	.write(true)
//         	.create(true)
//         	.open("dash.toml")
//         	.unwrap();

// 		println!("{:?}", contents);

//     	// convert Config to toml::Value to String
//     	// let toml_value = toml::Value::try_from(contents).unwrap();
// 		// let toml_string = toml::to_string_pretty(&toml_value).unwrap();

// 		//let toml_value = ron::Value::try_from(contents).unwrap();

// 		let pretty = ron::ser::PrettyConfig::new()
//     		.with_depth_limit(4)
//     		.with_indentor("\t".to_owned());

// 		let toml_string = ron::ser::to_string_pretty(&contents, pretty).unwrap();

//     	// convert string to byte & write to file
//     	match file.write_all(toml_string.as_bytes()) {
//         	Ok(_) => (),
//         	Err(_) => println!("error writing to file: {}.", "dash.toml"),
//     	}
// 	}
// }

// // Info about os and pkg/dotfiles
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Setup {
// 	pub os: String,
// 	pub distro: Option<String>,
// 	pub pkg_mgr: Option<String>,
// 	pub pkg: Vec<Package>,
// 	pub dotfile: Vec<Dotfile>,
// }

// // Package struct
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Package {
// 	pub name: String,
// 	pub prv_path: Option<String>,
// 	pub new_path: Option<String>,
// 	pub args: Option<Vec<String>>
// }

// // Dotfile struct
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Dotfile {
// 	pub name: String,
// 	pub prv_path: Option<String>,
// 	pub new_path: Option<String>,
// 	pub symlink: Option<bool>
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Command {
// 	pub cmd: String
// }