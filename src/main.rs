use wix::{args::Args, exit, question, WixConfig};

#[tokio::main]
async fn main() {

    // get default config.wix
    let wix_config = WixConfig::default();

    let args = Args::new(wix_config.clone());

    let path = match dirs::home_dir() {
        Some(path) => path.join("wix"),
        None => {
            eprintln!("Error: Could not find home directory.");
            exit!(1);
        }
    };

    if wix::setup::is_super() {
        eprintln!("Error: You are running wix as root.");
        eprintln!("Please run wix as a normal user to prevent damage.");
        exit!(1);
    }

    if !wix::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection.");
        exit!(1);
    }

    // check if config file exists
    if !path.clone().join("config.wix").exists() {
        // run setup?
        println!("{:#?}", wix_config.clone());
        if question!("Would you like to run setup?") {
            wix::setup::run(path.clone(), wix_config.clone(), args.clone());
        } else {
            exit!(1);
        }
    }

    // TODO: check if config.wix is valid and up to date

    // let mut pkgs: Vec<wix::pkg::Pkg> = Vec::new();

    // for arg_p in args.pkgs.clone() {
    //     let name = arg_p.0;
    //     let ver = arg_p.1;

    //     let pkg = wix::pkg::Pkg {
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
