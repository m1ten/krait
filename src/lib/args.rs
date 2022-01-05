use crate::{self as wix};
use clap::{App, Arg, SubCommand};
use indexmap::IndexMap;

// wix args struct
#[derive(Debug, Clone)]
pub struct Arguments {
    pub help: String,
    pub assume_yes: bool,
    pub status: String,
    pub pkgs: IndexMap<String, String>,
}

impl Arguments {
    // function to get wix args
    pub fn new(info: wix::Information) -> Arguments {
        let title = format!("- {} - {}", info.ver, info.desc);

        // get custom args
        let mut app = App::new(info.name.as_str())
            .version(title.as_str())
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
                    .args(&[
                        Arg::with_name("package")
                            .help("the package to install")
                            .takes_value(true)
                            .required(true)
                            .min_values(1)
                    ]),
            )
            .subcommand(
                SubCommand::with_name("uninstall")
                    .about("uninstall a package")
                    .visible_aliases(&["u", "un", "ui", "r", "rm", "remove"])
                    .arg(
                        Arg::with_name("package")
                            .help("package to uninstall")
                            .takes_value(true)
                            .required(true)
                            .min_values(1)
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
                            .required(true)
                            .min_values(1)
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
                            .required(true)
                            .min_values(1)
                    ),
            )
            .subcommand(
                SubCommand::with_name("clean")
                    .about("clean the cache")
                    .visible_aliases(&["cl", "cle", "cls", "clear"])
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
                    pkgs: matches
                        .subcommand_matches(l)
                        .unwrap()
                        .values_of("package")
                        .unwrap()
                        .map(|p| {
                            (
                                // remove everything after @ in the package name
                                p.split("@").next().unwrap().to_string(),

                                // get everything after @ in the package name
                                p.split("@").skip(1).next().unwrap().to_string(),
                            )
                        })
                        .collect(),
                };
            }
        }

        return Arguments {
            help: help,
            assume_yes: matches.is_present("assume-yes"),
            status: matches.subcommand_name().unwrap_or("").to_string(),
            pkgs: IndexMap::new(),
        };
    }
}
