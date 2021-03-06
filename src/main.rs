use krait::{args::Args, exit, kdbg, pkg::Pkg, question, KraitConfig};

#[tokio::main]
async fn main() {
    if krait::setup::is_super() {
        eprintln!("Error: You are running krait as root.");
        eprintln!("Please run krait as a normal user to prevent damage.");
        exit!(1);
    }

    if !krait::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection.");
        exit!(1);
    }

    let mut krait_config = KraitConfig::default();
    let args = Args::new(krait_config.clone());

    let krait_path = krait_config.dir.clone();
    let krait_path_yml = krait_path.clone().join("krait.yml");
    let krait_path_cache = krait_path.clone().join("cache");

    // check if config file exists
    if !krait_path_yml.exists() {
        // run setup?
        // println!("{:#?}", krait_config.clone());
        if question!("Would you like to run setup?") {
            krait::setup::run(krait_config);
            exit!(0);
        } else {
            exit!(1);
        }
    } else {
        // read config file
        let config_yaml = match krait::readfs(krait_path_yml.to_string_lossy().to_string()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: Reading krait.yml file: {}", e);
                eprintln!("Continuing with default config...");

                // struct to yaml
                serde_yaml::to_string(&KraitConfig::default())
                    .expect("Error: Could not convert krait config to yaml.")
            }
        };

        // convert yaml to struct

        krait_config = match serde_yaml::from_str(&config_yaml) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Error: Reading krait.yml file: '{}'", e);
                eprintln!("Continuing with default config...");

                KraitConfig::default()
            }
        }

        // TODO: check if config is valid and if not, run setup
    }

    let mut tasks = Vec::new();

    for arg_p in args.pkgs.clone() {
        let cache_dir = krait_config.dir.join("cache");
        let repos = krait_config.repos.clone();

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

    krait::kdbg!("finished");

    match args.status.as_str() {
        "search" => {
            println!("Finished!");
        }
        "clean" => {
            println!("Cleaning up.");

            kdbg!("{:#?}", &krait_path_cache);

            match std::fs::remove_dir_all(krait_path_cache) {
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
