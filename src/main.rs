use wix::{clear, exit, question, WixPy};

#[tokio::main]
async fn main() {
    let wix_py = WixPy::default();

    let args = wix::args::Arguments::new(info.clone());

    println!("Wix!\n");

    if wix::setup::is_super() {
        eprintln!("Error: You are running wix as root.");
        eprintln!("Please run wix as a normal user.");
        exit!(1);
    }

    if !wix::setup::is_python_installed("3.10") {
        eprintln!("Error: Python >=3.10 is not installed.");
        eprintln!("Please install and add Python to path.");
        exit!(127);
    }

    if !wix::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection.");
        exit!(1);
    }

    // check if config file exists
    if !dirs::home_dir().unwrap().join("wix/wix.py").exists() {
        // run setup?
        println!("{:?}", info.clone());
        if question!("Would you like to run setup?") {
            wix::setup::run(info.clone(), config.clone(), args.clone());
        } else {
            exit!(1);
        }
    }

    // TODO: check if wix.py is valid and up to date


    let mut pkgs: Vec<wix::pkg::Pkg> = Vec::new();

    for arg_p in args.pkgs.clone() {
        let name = arg_p.0;
        let ver = arg_p.1;

        let pkg = wix::pkg::Pkg {
            name,
            ver: Some(ver),
            ..Default::default()
        }
        .search()
        .await
        .unwrap();

        pkgs.push(pkg);
    }

    match args.status.as_str() {
        "search" => {
            println!("{:?}", pkgs);
            exit!(0);
        }
        "clean" => {
            println!("Cleaning up.");
            std::fs::remove_dir_all(dirs::home_dir().unwrap().join("wix/cache/")).unwrap_or_else(
                |err| {
                    eprintln!("Error Cleaning Cache: {}", err);
                    exit!(1);
                },
            );

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
