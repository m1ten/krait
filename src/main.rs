use wix::{args::Args, exit, pkg::Pkg, question, WixConfig};

#[tokio::main]
async fn main() {
    let path = match dirs::home_dir() {
        Some(path) => path.join("wix"),
        None => {
            eprintln!("Error: Could not find home directory.");
            exit!(1);
        }
    };

    let wix_config = WixConfig::default();

    let args = Args::new(wix_config.clone());

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
    if !path.clone().join("wix.yml").exists() {
        // run setup?
        // println!("{:#?}", wix_config.clone());
        if question!("Would you like to run setup?") {
            wix::setup::run(path.clone(), wix_config.clone(), args.clone());
            exit!(0);
        } else {
            exit!(1);
        }
    }

    // TODO: check if wix.yml is valid and up to date
    let mut tasks = Vec::new();

    for arg_p in args.pkgs.clone() {
        let cache_dir = wix_config.dir.cache_dir.clone();
        let repos = wix_config.info.repos.clone();

        tasks.push(tokio::spawn(async move {
            let name = arg_p.0;
            let ver = arg_p.1;

            let pkg = match (Pkg {
                name,
                ver,
                ..Default::default()
            })
            .fill(cache_dir, repos)
            .await
            {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit!(1);
                }
            };

            pkg
        }))
    }

    let mut pkgs = Vec::new();

    for r in futures::future::join_all(tasks).await {
        match r {
            Ok(p) => pkgs.push(p),
            Err(e) => {
                eprintln!("Error: {}", e);
                exit!(1);
            }
        }
    }

    dbg!("finished");

    match args.status.as_str() {
        "search" => {
            println!("Finished!");
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
