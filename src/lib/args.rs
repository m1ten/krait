use crate::{self as wix};
use clap::{App, Arg};
use indexmap::IndexMap;

// wix args struct
#[derive(Debug, Clone)]
pub struct Args {
    pub assume_yes: bool,
    pub status: String,
    pub pkgs: IndexMap<String, String>,
}

impl Args {
    // function to get wix args
    pub fn new(wix_config: wix::WixConfig) -> Args {
        let title = format!("- {} - {}", wix_config.gen.ver, wix_config.gen.desc);

        // get custom args
        let app = App::new(wix_config.gen.name.as_str())
            .version(title.as_str())
            .arg(
                Arg::new("assume-yes")
                    .short('y')
                    .long("assume-yes")
                    .value_name("Assume-Yes")
                    .help("yes to everything")
                    .takes_value(false),
            )
            .subcommand(
                App::new("install")
                    .about("install a package")
                    .visible_aliases(&["i", "in"])
                    .args(&[Arg::new("package")
                        .help("the package to install")
                        .takes_value(true)
                        .required(true)
                        .min_values(1)]),
            )
            .subcommand(
                App::new("uninstall")
                    .about("uninstall a package")
                    .visible_aliases(&["u", "un", "ui", "r", "rm", "remove"])
                    .arg(
                        Arg::new("package")
                            .help("package to uninstall")
                            .takes_value(true)
                            .required(true)
                            .min_values(1),
                    ),
            )
            .subcommand(
                App::new("search")
                    .about("search for a package")
                    .visible_aliases(&["s", "se", "sea"])
                    .arg(
                        Arg::new("package")
                            .help("package to search")
                            .takes_value(true)
                            .required(true)
                            .min_values(1),
                    ),
            )
            .subcommand(
                App::new("update")
                    .about("update a package")
                    .visible_aliases(&["up", "upd"])
                    .arg(
                        Arg::new("package")
                            .help("package to update")
                            .takes_value(true)
                            .required(true)
                            .min_values(1),
                    ),
            )
            .subcommand(
                App::new("clean")
                    .about("clean the cache")
                    .visible_aliases(&["cl", "cle", "cls", "clear"]),
            );

        let matches = app.get_matches();

        let list = vec!["install", "uninstall", "search"];

        for l in list {
            if matches.subcommand_matches(l).is_some() {
                return Args {
                    assume_yes: matches.is_present("assume-yes"),
                    status: l.to_string(),
                    pkgs: matches
                        .subcommand_matches(l)
                        .unwrap()
                        .values_of("package")
                        .unwrap()
                        .map(|p| -> (String, String) {
                            if p.contains("@") {
                                // remove everything after @ in the package name
                                (
                                    p.split("@").next().unwrap().to_string(),
                                    // get everything after @ in the package name
                                    p.split("@").skip(1).next().unwrap().to_string(),
                                )
                            } else {
                                (p.to_string(), "latest".to_string())
                            }
                        })
                        .collect(),
                };
            }
        }

        return Args {
            assume_yes: matches.is_present("assume-yes"),
            status: matches.subcommand_name().unwrap_or("").to_string(),
            pkgs: IndexMap::new(),
        };
    }
}
