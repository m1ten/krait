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

    // package install status (optional, default: false)
    pub installed: Option<bool>,

    // os (required)
    pub os: String,

    // arch (required)
    pub arch: String,
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
            os: wix::setup::get_os(),
            arch: wix::setup::get_arch(),
        }
    }
}

impl Pkg {
    // search self
    pub async fn search(mut self) -> Result<Pkg, String> {
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

            if self.os == "windows" {
                self.path = Some(self.path.unwrap().replace("/", "\\"));
            }
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
                        .await
                        .unwrap()
                        .text()
                        .await
                        .unwrap();

                    // write package script to local cache
                    let new_path = if self.os != "windows" {
                        self.path.clone().unwrap().replace(
                            &"/{ver}.py".replace("/{ver}", &self.ver.clone().unwrap()),
                            "",
                        )
                    } else {
                        self.path.clone().unwrap().replace(
                            &"\\{ver}.py".replace("\\{ver}", &self.ver.clone().unwrap()),
                            "",
                        )
                    };
                    std::fs::create_dir_all(new_path).unwrap();
                    let _ = wix::writefs(self.path.clone().unwrap(), res.clone());

                    if res.clone().contains("404: Not Found") {
                        println!(
                            "{}@{} not found.",
                            self.name.clone(),
                            self.ver.clone().unwrap()
                        );
                        return Err(res);
                    }

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
            "wix".to_string(),
            Some("installed_pkgs".to_string()),
            None,
        )
        .unwrap_or(std::collections::HashMap::new());

        self.installed = Some(installed_pkgs.contains_key(&self.name.clone()));

        // get type of package
        self._type = Some(
            wix::py::get_data::<String>(
                self.script.clone().unwrap(),
                self.path.clone().unwrap(),
                self.name.clone(),
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
                self.name.clone(),
                Some("verified".to_string()),
                None,
            )
            .unwrap_or(self.verified.clone().unwrap()),
        );

        // get package dependencies
        self.deps = match wix::py::get_data::<Vec<String>>(
            self.script.clone().unwrap(),
            wix_config.clone(),
            self.name.clone(),
            Some("deps".to_string()),
            None,
        ) {
            Ok(res) => Some(res),
            Err(_) => self.deps.clone(),
        };

        // get package description
        self.desc = match wix::py::get_data::<String>(
            self.script.clone().unwrap(),
            self.path.clone().unwrap(),
            self.name.clone(),
            Some("desc".to_string()),
            None,
        ) {
            Ok(res) => Some(res),
            Err(_) => self.desc.clone(),
        };

        // get package hash
        self.hash = match wix::py::get_data::<std::collections::HashMap<String, String>>(
            self.script.clone().unwrap(),
            self.path.clone().unwrap(),
            self.name.clone(),
            Some("hash".to_string()),
            None,
        ) {
            Ok(res) => Some(res),
            Err(_) => self.hash.clone(),
        };

        Ok(self)
    }

    // install self
    pub async fn install(self) {}
}
