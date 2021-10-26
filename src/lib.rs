use pyo3::prelude::*;
use std::io;
use std::fs::File;

#[pyfunction]
fn cmd(cmd: String, args: Vec<String>) -> PyResult<String> {
    let child = std::process::Command::new(cmd)
        .args(args)
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;

    Ok(String::from_utf8(output.stdout).unwrap())
}

#[pyfunction]
fn get(_py: Python, url: String, file: String) -> u64{
    let mut resp = reqwest::blocking::get(url).expect("Failed to get");
    let mut out = File::create(file).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy")
}

#[pymodule]
fn dash(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cmd, m)?)?;
    m.add_function(wrap_pyfunction!(get, m)?)?;

    Ok(())
}


// use std::fmt::Debug;
// use serde::{Serialize, Deserialize};
// use clap::{App, Arg};
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

// // dash args struct
// pub struct Arguments {
// 	pub confirm: bool,
// 	pub verbose: bool,
// }

// impl Arguments {
// 	// function to get dash args
// 	pub fn run(info: [(&str, &str); 4]) -> Arguments {
// 		// get custom args
// 		let matches = App::new(info[0].1)
// 			.version(info[1].1)
// 			.author(info[2].1)
// 			.about(info[3].1)
// 			.arg(
// 				Arg::with_name("no confirm")
// 					.short("y")
// 					.long("noconfirm")
// 					.value_name("NOCONFIRM")
// 					.help("yes to everything")
// 					.takes_value(false),
// 			)
// 			.arg(
// 				Arg::with_name("verbose")
// 					.short("v")
// 					.long("verbose")
// 					.value_name("Verbose")
// 					.help("print logs")
// 					.takes_value(false),
// 			)
// 			.get_matches();

// 		let mut data: Vec<bool> = Vec::new();
		
// 		match matches.occurrences_of("no confirm") {
// 			1 => data.push(true),
// 			_ => data.push(false),
// 		}
// 		match matches.occurrences_of("verbose") {
// 			1 => data.push(true),
// 			_ => data.push(false),
// 		}

// 		// convert vector string to struct arguments
// 		return Arguments {
// 			confirm: data[0],
// 			verbose: data[1],
// 		};
// 	}
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