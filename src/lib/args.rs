use crate as wix;
use clap::{App, Arg};

// wix args struct
#[derive(Debug)]
pub struct Arguments {
	pub confirm: bool,
	pub verbose: bool
}

impl Arguments {
	// function to get wix args
	pub fn new(info: wix::Information) -> Arguments {
		// get custom args
		let matches = App::new(info.name)
			.version(info.version.as_str())
			.author(info.author.as_str())
			.about(info.description.as_str())
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

		// convert vector string to struct arguments
		return Arguments {
			confirm: matches.is_present("no confirm"),
			verbose: matches.is_present("verbose")
		};
	}
}