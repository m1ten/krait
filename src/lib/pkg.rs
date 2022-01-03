use crate::{self as wix, exit};

#[derive(Debug, Clone)]
pub struct Pkg {
    // package name (e.g. "rust-lang")
    pub name: String,

    // package version (e.g. "1.0.0" or "latest")
    pub ver: Option<String>,

    // package script
    pub script: Option<String>,

    // package path (e.g. "wix/cache/rust-lang/latest.py")
    pub path: Option<String>,

    // package url (git, http, etc., default: git "wix-pkgs/rust-lang/latest.py")
    pub url: Option<String>,

    // package type (e.g. binary, git etc.) (optional, default: binary)
    pub _type: Option<String>,

    // package verification status (optional, default: false)
    pub verified: Option<bool>,

    // package dependencies (optional, default: none)
    pub deps: Option<Vec<String>>,

    // package description (optional, default: none)
    pub desc: Option<String>,

    // package hash with type as field name and value as field value (optional, default: none)
    pub hash: Option<std::collections::HashMap<String, String>>,

    // check if installed (optional, default: false)
    pub installed: Option<bool>,
}

impl Default for Pkg {
    fn default() -> Self {
        Pkg {
            name: String::new(),
            ver: Some("latest".to_string()),
            script: None,
            path: None,
            url: None,
            _type: Some("binary".to_string()),
            verified: Some(false),
            deps: None,
            desc: None,
            hash: None,
            installed: Some(false),
        }
    }
}

impl Pkg {
    // search self
    pub async fn search(mut self) -> Result<Self, reqwest::Error> {
        if self.name.is_empty() {
            println!("Package name is required.");
            exit!(1);
        }

        if self.path.is_none() {
            self.path = Some(
                dirs::home_dir()
                    .unwrap()
                    .join(
                        "wix/cache/{name}/{ver}.py"
                            .replace("{name}", &self.name)
                            .replace("{ver}", &self.ver.clone().unwrap()),
                    )
                    .to_str()
                    .unwrap()
                    .to_string(),
            );
        }

        if self.url.is_none() {
            self.url = Some(format!(
                "https://raw.githubusercontent.com/m1ten/wix-pkgs/main/{}/{}.py",
                self.name,
                self.ver.clone().unwrap()
            ));
        }

        // check if package exists in local cache
        if self.path.is_some() && self.script.is_none() {
            self.script = match std::fs::read_to_string(self.path.clone().unwrap()) {
                Ok(res) => Some(res),
                Err(_) => {
                    // clone package script from remote
                    let client = reqwest::Client::new();
                    let res = client
                        .get(self.url.clone().unwrap())
                        .header(reqwest::header::USER_AGENT, "Wix")
                        .send()
                        .await?
                        .text()
                        .await?;

                    // write package script to local cache
                    let _ = wix::writefs(self.path.clone().unwrap(), res.clone());

                    Some(res)
                }
            }
        }

        let wix_config = dirs::home_dir()
            .unwrap()
            .join("wix/wix.py")
            .to_str()
            .unwrap()
            .to_string();

        // check if package is installed

        let installed_pkgs = wix::py::get_data::<std::collections::HashMap<String, String>>(
            self.script.clone().unwrap(),
            wix_config.clone(),
            "installed_pkgs".to_string(),
            Some("installed_pkgs".to_string()),
            None,
        )
        .unwrap_or(std::collections::HashMap::new());

        self.installed = Some(installed_pkgs.contains_key(&self.name));

        // get type of package
        self._type = Some(
            wix::py::get_data::<String>(
                self.script.clone().unwrap(),
                self.path.clone().unwrap(),
                "type".to_string(),
                Some("type".to_string()),
                None,
            )
            .unwrap_or(self._type.clone().unwrap()),
        );

        // get package verified status
        self.verified = Some(
            wix::py::get_data::<bool>(
                self.script.clone().unwrap(),
                self.path.clone().unwrap(),
                "verified".to_string(),
                Some("verified".to_string()),
                None,
            )
            .unwrap_or(self.verified.clone().unwrap()),
        );

        // get package dependencies
        self.deps = Some(
            wix::py::get_data::<Vec<String>>(
                self.script.clone().unwrap(),
                wix_config.clone(),
                "deps".to_string(),
                Some("deps".to_string()),
                None,
            )
            .unwrap_or(self.deps.clone().unwrap()),
        );

        // get package description
        self.desc = Some(
            wix::py::get_data::<String>(
                self.script.clone().unwrap(),
                self.path.clone().unwrap(),
                "desc".to_string(),
                Some("desc".to_string()),
                None,
            )
            .unwrap_or(self.desc.clone().unwrap()),
        );

        // get package hash
        self.hash = Some(
            wix::py::get_data::<std::collections::HashMap<String, String>>(
                self.script.clone().unwrap(),
                self.path.clone().unwrap(),
                "hash".to_string(),
                Some("hash".to_string()),
                None,
            )
            .unwrap_or(self.hash.clone().unwrap()),
        );

        Ok(self)
    }

    // install self
    pub async fn install(self) {
        Package::install(self).await;
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
        // let folder = "{os}-{arch}".replace("{os}", &os).replace("{arch}", &arch);

        let url = "https://raw.githubusercontent.com/m1ten/wix-pkgs/main/{name}/{version}.py"
            .replace("{name}", &name)
            //.replace("{folder}", &folder)
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
            std::fs::create_dir_all(dirs::home_dir().unwrap().join(
                "wix/cache/{name}/".replace("{name}", &name), //.replace("{folder}", &folder),
            ))
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
                "wix/cache/{name}/{version}.py"
                    .replace("{name}", &name)
                    //.replace("{folder}", &folder)
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
    pub async fn install(pkg: Pkg) {
        println!("\nReview Script\n{}", pkg.script);

        let question = format!("Do you want to install {}?", pkg.name);

        if wix::question!(question) {
            println!("\nInstalling {}.\n", pkg.name);

            let function = wix::py::get_data::<bool>(
                pkg.script.clone(),
                pkg.path.clone(),
                pkg.name.clone(),
                None,
                Some("install".to_string()),
            );

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
            if installed_packages.contains_key(&pkg.name) {
                println!("{} is already installed.", pkg.name);
                exit!(1);
            }

            if function.unwrap_err().contains("TypeError: 'function'") {
                // call install function
                match wix::py::call_func(
                    pkg.script,
                    pkg.path,
                    pkg.name.clone(),
                    "install".to_string(),
                ) {
                    Ok(()) => println!("\n{} installed successfully.", pkg.name),
                    Err(e) => {
                        println!("\n{} failed to install.", pkg.name);
                        println!("{}", e);
                        exit!(1);
                    }
                }
            }

            // add package to installed_packages list
            installed_packages.insert(pkg.name.clone(), pkg.version.clone());

            let new_installed_packages =
                format!("installed_packages = {:?}", installed_packages.clone()).replace("\"", "'");

            // remove everything between { and } from config
            let start_bytes = config.find("installed_packages = {").unwrap();
            let end_bytes = config.find("}").unwrap();
            let mut new_config = config.clone();
            new_config.replace_range(start_bytes..end_bytes + 1, &new_installed_packages);

            // write new config
            wix::writefs(wix_path.clone(), new_config).unwrap();
        }
    }

    // function to uninstall package
    pub async fn uninstall(pkg: Pkg) {
        println!("\nReview Script\n{}", pkg.script);

        let question = format!("Do you want to uninstall {}?", pkg.name);

        if wix::question!(question) {
            println!("\nUninstalling {}.\n", pkg.name);

            let function = wix::py::get_data::<bool>(
                pkg.script.clone(),
                pkg.path.clone(),
                pkg.name.clone(),
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
            if !installed_packages.contains_key(&pkg.name) {
                println!("{} is not installed.", pkg.name);
                exit!(1);
            }

            if function.unwrap_err().contains("TypeError: 'function'") {
                // call install function
                match wix::py::call_func(
                    pkg.script,
                    pkg.path,
                    pkg.name.clone(),
                    "uninstall".to_string(),
                ) {
                    Ok(()) => println!("\n{} uninstalled successfully.", pkg.name),
                    Err(e) => {
                        println!("\n{} failed to uninstall.", pkg.name);
                        println!("{}", e);
                        exit!(1);
                    }
                }
            }

            // remove package from installed_packages list
            installed_packages.remove(&pkg.name);

            let new_installed_packages =
                format!("installed_packages = {:?}", installed_packages.clone()).replace("\"", "'");

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
