use crate::{self as wix, exit};

#[derive(Debug, Clone)]
pub struct Pkg {
    pub name: String,
    pub version: String,
    pub script: String,
    pub path: String,
}

impl Pkg {
    pub fn new(name: &str, version: &str, script: &str, path: &str) -> Pkg {
        Pkg {
            name: name.to_string(),
            version: version.to_string(),
            script: script.to_string(),
            path: path.to_string(),
        }
    }

    // install self 
    pub fn install(&self) {
        println!("Installing {:?}", self);   
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    // package name (e.g. "rust-lang/rust")
    pub name: String,

    // package version (e.g. "1.0.0")
    pub version: String,

    // package url (git, http, etc.)
    pub url: String,

    // package type (e.g. binary, git etc.) (optional, default: binary)
    pub _type: Option<String>,

    // package verification status (optional, default: false)
    pub verified: Option<bool>,

    // package dependency (optional, default: none)
    pub dependency: Option<String>,

    // package dependencies (optional, default: none)
    pub dependencies: Option<Vec<String>>,

    // package description (optional, default: none)
    pub description: Option<String>,

    // package hash with type as field name and value as field value (optional, default: none)
    pub hash: Option<std::collections::HashMap<String, String>>,
}

impl Package {
    // search for a package by name and version and return the package from github repo
    // (e.g. "rust", "1.0.0")
    pub async fn get_package(
        name: String,
        version: String,
        os: String,
        arch: String,
    ) -> Result<String, reqwest::Error> {
        let folder = "{os}-{arch}".replace("{os}", &os).replace("{arch}", &arch);

        let url =
            "https://raw.githubusercontent.com/m1ten/wix-pkgs/main/{name}/{folder}/{version}.py"
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

        let _ = wix::writefs(path, contents.clone());

        Ok(contents)
    }

    // TODO: add support for multiple packages

    // function to install package
    pub async fn install(pkgs: Vec<Pkg>) {
        for p in pkgs {
            println!("\nReview Script\n{}", p.script);

            let question = format!("Do you want to install {}?", p.name);

            if wix::question!(question) {
                println!("\nInstalling {}.\n", p.name);

                let function = wix::py::get_data::<bool>(
                    p.script.clone(),
                    p.path.clone(),
                    p.name.clone(),
                    None,
                    Some("install".to_string()),
                );

                // TODO: add support for installing packages with no install function

                // get installed_packages list from config
                // add package to installed_packages list

                // read config
                let wix_path = dirs::home_dir()
                    .unwrap()
                    .join("wix/wix.py")
                    .to_str()
                    .unwrap()
                    .to_string();
                let config = wix::readfs(wix_path.clone()).unwrap();

                // get installed_packages list
                let mut installed_packages =
                    wix::py::get_data::<std::collections::HashMap<String, String>>(
                        config.clone(),
                        wix_path.clone(),
                        "wix".to_string(),
                        Some("installed_packages".to_string()),
                        None,
                    )
                    .unwrap();

                // check if package is already installed
                if installed_packages.contains_key(&p.name) {
                    println!("{} is already installed.", p.name);
                    exit!(1);
                }

                if function.unwrap_err().contains("TypeError: 'function'") {
                    // call install function
                    match wix::py::call_func(
                        p.script,
                        p.path,
                        p.name.clone(),
                        "install".to_string(),
                    ) {
                        Ok(()) => println!("\n{} installed successfully.", p.name),
                        Err(e) => {
                            println!("\n{} failed to install.", p.name);
                            println!("{}", e);
                            exit!(1);
                        }
                    }
                }

                // add package to installed_packages list
                installed_packages.insert(p.name.clone(), p.version.clone());

                let new_installed_packages =
                    format!("installed_packages = {:?}", installed_packages.clone())
                        .replace("\"", "'");

                // remove everything between { and } from config
                let start_bytes = config.find("installed_packages = {").unwrap();
                let end_bytes = config.find("}").unwrap();
                let mut new_config = config.clone();
                new_config.replace_range(start_bytes..end_bytes + 1, &new_installed_packages);

                // write new config
                wix::writefs(wix_path.clone(), new_config).unwrap();
            }
        }
    }

    // function to uninstall package
    pub async fn uninstall(pkgs: Vec<Pkg>) {
        for p in pkgs {
            println!("\nReview Script\n{}", p.script);

            let question = format!("Do you want to uninstall {}?", p.name);

            if wix::question!(question) {
                println!("\nUninstalling {}.\n", p.name);

                let function = wix::py::get_data::<bool>(
                    p.script.clone(),
                    p.path.clone(),
                    p.name.clone(),
                    None,
                    Some("uninstall".to_string()),
                );

                // TODO: add support for uninstalling packages with no uninstall function

                // get installed_packages list from config
                // remove package from installed_packages list

                // read config
                let wix_path = dirs::home_dir()
                    .unwrap()
                    .join("wix/wix.py")
                    .to_str()
                    .unwrap()
                    .to_string();
                let config = wix::readfs(wix_path.clone()).unwrap();

                // get installed_packages list
                let mut installed_packages =
                    wix::py::get_data::<std::collections::HashMap<String, String>>(
                        config.clone(),
                        wix_path.clone(),
                        "wix".to_string(),
                        Some("installed_packages".to_string()),
                        None,
                    )
                    .unwrap();

                // check if package is already installed
                if !installed_packages.contains_key(&p.name) {
                    println!("{} is not installed.", p.name);
                    exit!(1);
                }

                if function.unwrap_err().contains("TypeError: 'function'") {
                    // call install function
                    match wix::py::call_func(p.script, p.path, p.name.clone(), "uninstall".to_string()) {
                        Ok(()) => println!("\n{} uninstalled successfully.", p.name),
                        Err(e) => {
                            println!("\n{} failed to uninstall.", p.name);
                            println!("{}", e);
                            exit!(1);
                        }
                    }
                }

                // remove package from installed_packages list
                installed_packages.remove(&p.name);

                let new_installed_packages =
                    format!("installed_packages = {:?}", installed_packages.clone())
                        .replace("\"", "'");

                // remove everything between { and } from config
                let start_bytes = config.find("installed_packages = {").unwrap();
                let end_bytes = config.find("}").unwrap();
                let mut new_config = config.clone();
                new_config.replace_range(start_bytes..end_bytes + 1, &new_installed_packages);

                // write new config
                wix::writefs(wix_path.clone(), new_config).unwrap();
            }
        }
    }
}
