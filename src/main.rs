use krait::{
    args::Args, exit, kdbg, question, scripts::KraitScript, structs::config::KraitConfig,
    structs::pkg::Pkg, structs::KraitMain,
};

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

    let mut krait_main = KraitMain {
        config: Some(KraitConfig::default()),
        ..Default::default()
    };

    let args = Args::new(krait_main.config.as_ref().unwrap());

    let krait_path_lua = krait_main.config.as_ref().unwrap().dir.join("krait.lua");
    let krait_path_cache = krait_main.config.as_ref().unwrap().dir.join("cache");

    // check if config file exists
    if krait_path_lua.exists() && krait_path_lua.metadata().unwrap().len() == 0 {
        if question!("Would you like to reset krait?") {
            krait::setup::run(&krait_main);
            exit!(0);
        }
    } else {
        // read config file
        let mut config_str = match krait::readfs(krait_path_lua.to_string_lossy().to_string()) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("Error: Reading krait.lua file: {}", e);

                if question!("Would you like to reset krait?") {
                    krait::setup::run(&krait_main);
                    exit!(0);
                } else {
                    exit!(1);
                }
            }
        };

        config_str = match regex::Regex::new(r"^\s*$") {
            Ok(x) => x.replace_all(&config_str, "").to_string(),
            Err(e) => {
                eprintln!("Error: Regex: {}", e);

                if question!("Would you like to reset krait?") {
                    krait::setup::run(&krait_main);
                    exit!(0);
                } else {
                    exit!(1);
                }
            }
        };

        // TODO: parse config file
        krait_main.config = match KraitConfig::parse(&config_str) {
            Ok(m) => match m.config {
                Some(c) => Some(c),
                None => {
                    eprintln!("Error: Parsing krait.lua file: No config found");

                    if question!("Would you like to reset krait?") {
                        krait::setup::run(&krait_main);
                        exit!(0);
                    } else {
                        exit!(1);
                    }
                }
            },
            Err(e) => {
                eprintln!("Error: Parsing krait.lua file: {}", e);

                if question!("Would you like to reset krait?") {
                    krait::setup::run(&krait_main);
                    exit!(0);
                } else {
                    exit!(1);
                }
            }
        };
    }

    kdbg!(&krait_main);

    let mut tasks = Vec::new();

    for arg_p in args.pkgs.clone() {
        let cache_dir = krait_path_cache.clone();
        let repos = krait_main.config.as_ref().unwrap().repos.clone();

        tasks.push(tokio::spawn(async move {
            let name = arg_p.0;
            let ver = arg_p.1;

            let pkg = Pkg {
                name,
                ver,
                ..Default::default()
            }
            .fill(&cache_dir, &repos)
            .await;

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
            // ask if they want to clean the whole folder

            if question!("Would you like to clean the krait directory?") {
                // clean the whole folder
                println!("Cleaning krait folder...");

                kdbg!("{:#?}", krait_path_cache);

                match std::fs::remove_dir_all(krait_main.config.unwrap().dir) {
                    Ok(_) => {
                        println!("Krait directory cleaned!");
                    }
                    Err(_) => {
                        eprintln!("Error: Could not remove krait directory.");
                        exit!(1);
                    }
                }
            } else {
                println!("Cleaning up.");

                kdbg!("{:#?}", &krait_path_cache);

                match std::fs::remove_dir_all(krait_path_cache) {
                    Ok(_) => {
                        println!("Cache cleaned!");
                        exit!(0);
                    }
                    Err(_) => {
                        eprintln!("Error: Could not remove cache directory.");
                        exit!(1);
                    }
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
