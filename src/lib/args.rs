use crate::{self as wix};
use clap::{App, Arg, SubCommand};

// wix args struct
#[derive(Debug, Clone)]
pub struct Arguments {
	pub help: String,
    pub assume_yes: bool,
    pub status: String,
    pub package: String,
}

impl Arguments {
    // function to get wix args
    pub fn new(info: wix::structs::Information) -> Arguments {
        // get custom args
        let mut app = App::new(info.name)
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
            .subcommand(
                SubCommand::with_name("install")
                    .about("install a package")
                    .visible_aliases(&["i", "in"])
                    .arg(
                        Arg::with_name("package")
                            .help("package to install")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("uninstall")
                    .about("uninstall a package")
                    .visible_aliases(&["u", "un", "ui", "r", "rm", "remove"])
                    .arg(
                        Arg::with_name("package")
                            .help("package to uninstall")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("search")
                    .about("search for a package")
                    .visible_aliases(&["s", "se", "sea"])
                    .arg(
                        Arg::with_name("package")
                            .help("package to search")
                            .takes_value(true)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("update")
                    .about("update a package")
                    .visible_aliases(&["up", "upd"])
                    .arg(
                        Arg::with_name("package")
                            .help("package to update")
                            .takes_value(true)
                            .required(true),
                    ),
            );

        let mut help = Vec::new();
        app.write_long_help(&mut help).unwrap();
		let help = String::from_utf8(help).unwrap();
        let matches = app.get_matches();

        let list = vec!["install", "uninstall", "search"];

        for l in list {
            if matches.subcommand_matches(l).is_some() {
                return Arguments {
					help: help,
                    assume_yes: matches.is_present("assume-yes"),
                    status: l.to_string(),
                    package: matches
                        .subcommand_matches(l)
                        .unwrap()
                        .value_of("package")
                        .unwrap()
                        .to_string(),
                };
            }
        }

        // convert vector string to struct arguments
        return Arguments {
			help: help,
            assume_yes: matches.is_present("assume-yes"),
            status: "".to_string(),
            package: "".to_string(),
        };
    }
}
