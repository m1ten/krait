use wix::{exit, question, clear};

fn main() {
    let info = wix::structs::Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "wix - cross platform package manager".to_string(),
        license: "zlib".to_string(),
        repository: "https://github.com/m1ten/wix".to_string(),
    };

    let args = wix::args::Arguments::new(info.clone());

    println!("Wix!\n");

    if wix::setup::is_super() {
        eprintln!("{}", "Error: You are running wix as root.");
        exit!(1);
    }

    if !wix::setup::is_python_installed() {
        eprintln!("Error: Python >=3.8 is not installed.");
        eprintln!("Please install and add Python to path then try again.");
        exit!(127);
    }

    // check if config file exists
    if !dirs::home_dir().unwrap().join("wix/wix.py").exists() {
        // run setup?
        if question!("Would you like to run setup?") {
            wix::setup::run(info.clone(), args.clone());
        } else {
            exit!(1);
        }
    }

    // check if wix.py is up to date

    match args.status.as_str() {
        "install" => println!("Installing {}", args.package),
        "uninstall" => println!("Uninstalling {}", args.package),
        "search" => println!("Searching for {}", args.package),
        "update" => println!("Updating {}", args.package),
        _ => {
            clear!();
            println!("{}", args.help);
            exit!(1);
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
