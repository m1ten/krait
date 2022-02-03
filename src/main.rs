use wix::{clear, exit, question, WixConfig};

#[tokio::main]
async fn main() {
    
    // get default wix.toml
    let wix_config = WixConfig::default();

    let args = wix::args::Arguments::new(wix_config.clone());

    let path = match dirs::home_dir() {
        Some(path) => path,
        None => {
            eprintln!("Error: Could not find home directory.");
            exit!(1);
        }
    };

    println!("Wix!\n");

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
    if !path.clone().join("wix.py").exists() {
        // run setup?
        println!("{:#?}", wix_config.clone());
        if question!("Would you like to run setup?") {
            wix::setup::run(path.clone(), wix_config.clone(), args.clone());
        } else {
            exit!(1);
        }
    }

    if !wix::setup::is_venv() {
        eprintln!("Error: wix is not in a virtual environment.");
        if question!("Would you like to create a venv?") {
           wix::setup::venv(path.clone().join("venv"));
        } else {
            exit!(1);
        }
    }

    // TODO: check if wix.py is valid and up to date


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
            // std::fs::remove_dir_all(dirs::home_dir().unwrap().join("wix/cache/")).unwrap_or_else(
            //     |err| {
            //         eprintln!("Error Cleaning Cache: {}", err);
            //         exit!(1);
            //     },
            // );

            println!("Cache Cleaned!");
            exit!(0);
        }
        _ => {
            clear!();
            println!("{}", args.help);
            exit!(0);
        }
    }
}
