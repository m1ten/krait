use neo::{args::Args, exit, question, NeoConfig};

#[tokio::main]
async fn main() {
    let path = match dirs::home_dir() {
        Some(path) => path.join("neopkg"),
        None => {
            eprintln!("Error: Could not find home directory.");
            exit!(1);
        }
    };

    // get default config.neo
    let neo_config = NeoConfig {
        dir: neo::NeoDir {
            dir: path.clone(),
            bin_dir: path.clone().join("bin"),
            cache_dir: path.clone().join("cache"),
            temp_dir: path.clone().join("temp"),
        },
        ..Default::default()
    };

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
