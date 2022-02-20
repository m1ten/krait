use std::collections::HashMap;

use neo::{args::Args, exit, question, NeoConfig, NeoInfo};

#[tokio::main]
async fn main() {
    let package = neo::pkg::PkgInfo {
        name: "dotfiler".to_string(),
        ver: "0.1.0".to_string(),
        desc: Some("A simple command line utility for managing your dotfiles.".to_string()),
        license: Some(vec!["MIT".to_string(), "Apache-2.0".to_string()]),
        pkg: Some(vec![
            neo::pkg::PkgMain {
                supports: Some(HashMap::from([
                    (String::from("any"), vec![String::from("posix")]),
                    (
                        String::from("x86"),
                        vec![String::from("linux"), String::from("darwin")],
                    ),
                ])),
                repos: Some(vec!["https://github.com/dotfiler/dotfiler".to_string()]),
                deps: Some(vec!["rust".to_string(), "toml".to_string()]),
                dev_deps: Some(vec!["cargo".to_string(), "rustfmt".to_string()]),
                _type: String::from("binary"),
                srcs: Some(vec![neo::pkg::PkgSrc {
                    urls: vec![
                        "dotfiler.com/dotfiler.sh".to_string(),
                        "sketchymirror.com/dotfiler.sh".to_string(),
                    ],
                    hashes: Some(HashMap::from([
                        (String::from("sha256"), String::from("0x0")),
                        (String::from("md5"), String::from("0x0")),
                    ])),
                }]),
                install: None,
                uninstall: None,
            },
            neo::pkg::PkgMain {
                supports: Some(HashMap::from([
                    (String::from("any"), vec![String::from("windows")]),
                    (
                        String::from("x86"),
                        vec![String::from("10"), String::from("11")],
                    ),
                ])),
                repos: Some(vec!["https://github.com/dotfiler/dotfiler".to_string()]),
                deps: Some(vec!["rust".to_string(), "toml".to_string()]),
                dev_deps: Some(vec!["cargo".to_string(), "rustfmt".to_string()]),
                _type: String::from("binary"),
                srcs: Some(vec![neo::pkg::PkgSrc {
                    urls: vec![
                        "dotfiler.com/dotfiler.exe".to_string(),
                        "sketchymirror.com/dotfiler.exe".to_string(),
                    ],
                    hashes: Some(HashMap::from([
                        (String::from("sha256"), String::from("0x0")),
                        (String::from("md5"), String::from("0x0")),
                    ])),
                }]),
                install: None,
                uninstall: None,
            },
        ]),
    };

    // package to yaml
    let yaml = serde_yaml::to_string(&package).unwrap();

    let yaml = r#"

name: dotfiler
ver: 0.1.0
desc: A simple command line utility for managing your dotfiles.
license:
  - MIT
  - Apache-2.0
pkg:
  - supports:
      any:
        - posix
      x86:
        - linux
        - darwin
    repos:
      - "https://github.com/dotfiler/dotfiler"
    deps:
      - rust
      - toml
    dev_deps:
      - cargo
      - rustfmt
    type: binary
    srcs:
      - urls:
          - dotfiler.com/dotfiler.sh
          - sketchymirror.com/dotfiler.sh
        hashes:
          md5: "0x0"
          sha256: "0x0"
  - supports:
      any:
        - windows
      x86:
        - "10"
        - "11"
    repos:
      - "https://github.com/dotfiler/dotfiler"
    deps:
      - rust
      - toml
    dev_deps:
      - cargo
      - rustfmt
    type: binary
    srcs: null

"#
    .to_string();

    // yaml to package
    let package: neo::pkg::PkgInfo = serde_yaml::from_str(&yaml).unwrap();

    // println!("{:#?}", package);

    let bin_str = r#"

name: dotfiler
nombre: \*name + dotfiler

"#.to_string();

    let binary = yaml_rust::YamlLoader::load_from_str(&bin_str).unwrap();

    println!("{:#?}", binary);

    // bat::PrettyPrinter::new()
    //     .input_from_bytes(yaml.as_bytes())
    //     .language("yaml")
    //     .line_numbers(false)
    //     .grid(true)
    //     .theme("Visual Studio Dark+")
    //     .print()
    //     .expect("Error: Could not print yaml.");

    return;

    let path = match dirs::home_dir() {
        Some(path) => path.join("neopkg"),
        None => {
            eprintln!("Error: Could not find home directory.");
            exit!(1);
        }
    };

    let neo_config = NeoConfig::default();

    let args = Args::new(neo_config.clone());

    if neo::setup::is_super() {
        eprintln!("Error: You are running neopkg as root.");
        eprintln!("Please run neopkg as a normal user to prevent damage.");
        exit!(1);
    }

    if !neo::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection.");
        exit!(1);
    }

    // check if config file exists
    if !path.clone().join("neopkg.yml").exists() {
        // run setup?
        // println!("{:#?}", neo_config.clone());
        if question!("Would you like to run setup?") {
            neo::setup::run(path.clone(), neo_config.clone(), args.clone());
            exit!(0);
        } else {
            exit!(1);
        }
    }

    // TODO: check if neopkg.yml is valid and up to date

    // let mut pkgs: Vec<neo::pkg::Pkg> = Vec::new();

    // for arg_p in args.pkgs.clone() {
    //     let name = arg_p.0;
    //     let ver = arg_p.1;

    //     let pkg = neo::pkg::Pkg {
    //         name,
    //         ver: Some(ver),
    //         ..Default::default()
    //     }
    //     .search()
    //     .await
    //     .unwrap();

    //     pkgs.push(pkg);
    // }

    match args.status.as_str() {
        "search" => {
            // println!("{:?}", pkgs);
            exit!(0);
        }
        "clean" => {
            println!("Cleaning up.");

            let cache = path.clone().join("cache");

            println!("{:#?}", cache);

            match std::fs::remove_dir_all(path.clone().join("cache")) {
                Ok(_) => {
                    println!("Cache Cleaned!");
                    exit!(0);
                }
                Err(_) => {
                    println!("Error: Could not remove cache directory.");
                    exit!(1);
                }
            }
        }
        _ => {
            // call self exe with arg '--help'
            let mut cmd = std::process::Command::new(std::env::current_exe().unwrap());
            cmd.arg("--help");
            cmd.spawn()
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    exit!(1);
                })
                .wait()
                .unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    exit!(1);
                });
        }
    }
}
