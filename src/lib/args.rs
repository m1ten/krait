use clap::{App, Arg};

// wix args struct
pub struct Arguments {
	pub confirm: bool,
	pub verbose: bool,
    pub file: String,
}

impl Arguments {
	// function to get wix args
	pub fn run(info: [(&str, &str); 4]) -> Arguments {
		// get custom args
		let matches = App::new(info[0].1)
			.version(info[1].1)
			.author(info[2].1)
			.about(info[3].1)
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
            .arg(
                Arg::with_name("file")
                    .short("f")
                    .long("file")
                    .value_name("FILE")
                    .help("file to read")
                    .takes_value(true),
            )
			.get_matches();

		// convert vector string to struct arguments
		return Arguments {
			confirm: matches.is_present("no confirm"),
			verbose: matches.is_present("verbose"),
            file: matches.value_of("file").unwrap().to_string(),
		};
	}
}