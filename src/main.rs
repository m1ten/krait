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

    match args.status.as_str() {
        "install" => {
            let os = "windows".to_string();
            let version = "latest".to_string();
            let arch = "x86_64".to_string();
            let path = dirs::home_dir()
                .unwrap()
                .join("wix/cache/{name}/{os}-{arch}/{version}.py")
                .to_str()
                .unwrap()
                .to_string()
                .replace("{name}", args.clone().package.as_str())
                .replace("{os}", os.as_str())
                .replace("{arch}", arch.as_str())
                .replace("{version}", version.as_str());

            let package = get_package(
                args.package.to_lowercase(),
                version.clone(),
                os.clone(),
                arch.clone(),
            )
            .await
            .unwrap();

            match package.as_str() {
                "404: Not Found" => {
                    eprintln!("Error: Package not found in repository.");
                    exit!(1);
                }
                _ => wix::structs::Package::install(package, args.package.clone(), path)
            }
        }
        "uninstall" => println!("Uninstalling {}", args.package),
        "search" => println!("Searching for {}", args.package),
        "update" => println!("Updating {}", args.package),
        _ => {
            clear!();
            println!("{}", args.help);
            exit!(0);
        }
    }

    // match std::env::var("USER") {
    //     Ok(user) => {
    //         match user.as_str() {
    //             "root" => { eprintln!("Please run wix as a regular user."); return },
    //             _ => (),
    //         }
    //     },
    //     Err(e) => ()
    // }

    // let info_contents = wix::structs::Information::get_field_type(Some(info));

    // let mut info_code = wix::lang::struct_to_py("Information".to_string(), info_contents);
    // info_code = info_code.replace("Information = {}", "").replace("Information.", "");

    // wix::file::writefs(
    //     "wix.py".to_string(),
    //     info_code.trim_start().to_string().trim_end().to_string()
    // ).unwrap();
}

// search for a package by name and version and return the package from github repo
// (e.g. "rust", "1.0.0")
pub async fn get_package(
    name: String,
    version: String,
    os: String,
    arch: String,
) -> Result<String, reqwest::Error> {
    let folder = "{os}-{arch}".replace("{os}", &os).replace("{arch}", &arch);

    let url = "https://raw.githubusercontent.com/m1ten/wix-pkgs/main/{name}/{folder}/{version}.py"
        .replace("{name}", &name)
        .replace("{folder}", &folder)
        .replace("{version}", &version);

    let client = reqwest::Client::new();
    let contents = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Wix")
        .send()
        .await?
        .text()
        .await?;

    // check if package exists in local cache
    if !dirs::home_dir().unwrap().join("wix/cache/{name}").exists() {
        // create cache folder
        std::fs::create_dir_all(
            dirs::home_dir().unwrap().join(
                "wix/cache/{name}/{folder}/"
                    .replace("{name}", &name)
                    .replace("{folder}", &folder),
            ),
        )
        .unwrap();
    } else {
        // check if package exists in cache
        // if !dirs::home_dir().unwrap().join(
        //     "wix/cache/{name}/{folder}/{version}.py"
        //         .replace("{name}", &name)
        //         .replace("{folder}", &folder)
        //         .replace("{version}", &version),
        // )
        // .exists()
        // {

        // }
    }

    let path = dirs::home_dir()
        .unwrap()
        .join(
            "wix/cache/{name}/{folder}/{version}.py"
                .replace("{name}", &name)
                .replace("{folder}", &folder)
                .replace("{version}", &version),
        )
        .to_str()
        .unwrap()
        .to_string();

    writefs(path, contents.clone());

    Ok(contents)
}
