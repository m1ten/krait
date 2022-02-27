use neopkg::{args::Args, exit, question, NPConfig};

#[tokio::main]
async fn main() {
    let path = match dirs::home_dir() {
        Some(path) => path.join("neopkg"),
        None => {
            eprintln!("Error: Could not find home directory.");
            exit!(1);
        }
    };

    let np_config = NPConfig::default();

    let args = Args::new(np_config.clone());

    if neopkg::setup::is_super() {
        eprintln!("Error: You are running neopkg as root.");
        eprintln!("Please run neopkg as a normal user to prevent damage.");
        exit!(1);
    }

    if !neopkg::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection.");
        exit!(1);
    }

    // check if config file exists
    if !path.clone().join("neopkg.yml").exists() {
        // run setup?
        // println!("{:#?}", np_config.clone());
        if question!("Would you like to run setup?") {
            neopkg::setup::run(path.clone(), np_config.clone(), args.clone());
            exit!(0);
        } else {
            exit!(1);
        }
    }

    // TODO: check if neopkg.yml is valid and up to date

    // let mut pkgs: Vec<neopkg::pkg::Pkg> = Vec::new();

    // for arg_p in args.pkgs.clone() {
    //     let name = arg_p.0;
    //     let ver = arg_p.1;

    //     let pkg = neopkg::pkg::Pkg {
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
