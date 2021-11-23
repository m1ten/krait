use wix::{clear, exit, question, structs::Configuration, writefs};

#[tokio::main]
async fn main() {
    let config: Configuration = wix::structs::Configuration {
        repo: "https:://github.com/m1ten/wix-pkgs".to_string(),
        mirror: None,
    };

    let info = wix::structs::Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "wix - cross platform package manager".to_string(),
        license: "zlib".to_string(),
        git: "https://github.com/m1ten/wix".to_string(),
    };

    let args = wix::args::Arguments::new(info.clone());

    println!("Wix!\n");

    if wix::setup::is_super() {
        eprintln!("{}", "Error: You are running wix as root.");
        eprintln!("{}", "Please run wix as a normal user.");
        exit!(1);
    }

    if !wix::setup::is_python_installed() {
        eprintln!("Error: Python >=3.8 is not installed.");
        eprintln!("Please install and add Python to path then try again.");
        exit!(127);
    }

    if !wix::setup::is_internet_connected().await {
        eprintln!("Error: Internet connection is not available.");
        eprintln!("Please check your internet connection and try again.");
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

    // check if wix.py is up to date

    let os = "windows".to_string();
    let version = "latest".to_string();
    let arch = "x86_64".to_string();
    let mut path = dirs::home_dir()
        .unwrap()
        .join("wix/cache/{name}/{os}-{arch}/{version}.py")
        .to_str()
        .unwrap()
        .to_string()
        .replace("{name}", args.clone().package.as_str())
        .replace("{os}", os.as_str())
        .replace("{arch}", arch.as_str())
        .replace("{version}", version.as_str());

    if cfg!(windows) {
        path = path.replace("/", "\\");
    }

    let package = wix::structs::Package::get_package(
        args.package.to_lowercase(),
        version.clone(),
        os.clone(),
        arch.clone(),
    )
    .await
    .unwrap();

    match args.status.as_str() {
        "install" => {
            match package.as_str() {
                "404: Not Found" => {
                    eprintln!("Error: Package not found in repository.");
                    exit!(1);
                }
                _ => wix::structs::Package::install(package, args.package.clone(), path),
            }
        }
        "uninstall" => {
            match package.as_str() {
                "404: Not Found" => {
                    eprintln!("Error: Package not found in repository.");
                    exit!(1);
                }
                _ => wix::structs::Package::uninstall(package, args.package.clone(), path),
            }
        },
        "search" => {
            match package.as_str() {
                "404: Not Found" => {
                    eprintln!("Error: Package not found in repository.");
                    exit!(1);
                }
                _ => {
                    println!("{} cloned to path '{}'.\nReview Script\n{}", args.package, path, package);
                    exit!(0);
                }
            }
        },
        "update" => println!("Updating {}", args.package),
        _ => {
            clear!();
            println!("{}", args.help);
            exit!(0);
        }
    }
}