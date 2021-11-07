use crate as wix;
use clap::{App, Arg};

// wix args struct
#[derive(Debug)]
pub struct Arguments {
	pub verbose: bool,
	pub package: String,
	pub file: String
}

impl Arguments {
	// function to get wix args
	pub fn new(info: wix::structs::Information) -> Arguments {
		// get custom args
		let matches = App::new(info.name)
			.version(info.version.as_str())
			.author(info.author.as_str())
			.about(info.description.as_str())
			.arg(
				Arg::with_name("assume-yes")
					.short("y")
					.long("assume-yes")
					.value_name("Assume-Yes")
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
			.arg(
				Arg::with_name("package")
					.short("p")
					.long("package")
					.value_name("Package")
					.help("package name")
					.takes_value(true),
			)
			.arg(
				Arg::with_name("file")
					.short("f")
					.long("file")
					.value_name("File")
					.help("file name")
					.takes_value(true),
			)
			.get_matches();

		// convert vector string to struct arguments
		return Arguments {
			verbose: matches.is_present("verbose"),
			package: matches.value_of("package").unwrap_or("").to_string(),
			file: matches.value_of("file").unwrap_or("").to_string(),
		};
	}
}