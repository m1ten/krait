use wix::{clear, exit, pkg, question, Configuration, Information};

#[tokio::main]
async fn main() {
    let config: Configuration = Configuration {
        repo: "https:://github.com/m1ten/wix-pkgs".to_string(),
        mirror: None,
    };

    let info = Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "cross platform package manager".to_string(),
        license: "zlib".to_string(),
        git: "https://github.com/m1ten/wix".to_string(),
    };

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

    // check if wix.py is up to date
    let mut pkgs: Vec<wix::pkg::Pkg> = vec![];
    let os = wix::setup::get_os();
    let arch = wix::setup::get_arch();

    for pkg in args.package.clone() {
        let pkg_name = pkg.0;
        let pkg_version = pkg.1;

        let mut pkg_path = dirs::home_dir()
            .unwrap()
            .join("wix/cache/{name}/{os}-{arch}/{version}.py")
            .to_str()
            .unwrap()
            .to_string()
            .replace("{name}", pkg_name.as_str())
            .replace("{os}", os.as_str())
            .replace("{arch}", arch.as_str())
            .replace("{version}", pkg_version.as_str());

        if cfg!(windows) {
            pkg_path = pkg_path.replace("/", "\\");
        }

        let pkg_script = if !pkg_name.is_empty() {
            pkg::Package::get_package(
                pkg_name.clone().to_lowercase(),
                pkg_version.clone(),
                os.clone(),
                arch.clone(),
            )
            .await
            .unwrap()
        } else {
            String::from("")
        };

        pkgs.push(wix::pkg::Pkg {
            name: pkg_name.clone(),
            version: pkg_version.clone(),
            script: pkg_script,
            path: pkg_path.clone(),
        });
    }

    for p in pkgs.clone() {
        if p.script == "404: Not Found" {
            eprintln!("Error: Package {} not found.", p.name);
            exit!(1);
        }
    }

    match args.status.as_str() {
        "install" => {
            for p in pkgs {
                p.install().await;
            }
        }
        // "install" => pkg::Package::install(pkgs.clone()).await,
        // "uninstall" => pkg::Package::uninstall(pkgs.clone()).await,
        "search" => println!("Searching"),
        "update" => println!("Updating"),
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
