use crate::{self as wix};

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
    pub urls: Option<Vec<String>>,

    // package type (e.g. binary, git etc.) (optional, default: binary)
    pub _type: Option<String>,

    // package verification status (optional, default: false)
    pub verified: Option<bool>,

    // package dependencies (optional, default: none)
    pub deps: Option<Vec<String>>,

    // package build dependencies (optional, default: none)
    pub build_deps: Option<Vec<String>>,

    // package description (optional, default: none)
    pub desc: Option<String>,

    // package hash with type as field name and value as field value (optional, default: none)
    pub hash: Option<std::collections::HashMap<String, String>>,

    // package install status (optional, default: false)
    pub installed: Option<bool>,

    // supported platforms (optional, default: all)
    pub platforms: Option<Vec<String>>,

    // provides (optional, default: none)
    pub provides: Option<Vec<String>>,

    // conflicts (optional, default: none)
    pub conflicts: Option<Vec<String>>,

    // maintainer (optional, default: none)
    pub maintainers: Option<Vec<String>>,

    // license (optional, default: none)
    pub license: Option<String>,

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
            urls: None,
            _type: Some("binary".to_string()),
            verified: Some(false),
            deps: None,
            build_deps: None,
            desc: None,
            hash: None,
            installed: Some(false),
            platforms: None,
            provides: None,
            conflicts: None,
            maintainers: None,
            license: None,
            os: wix::setup::get_os(),
            arch: wix::setup::get_arch(),
        }
    }
}

impl Pkg {
    // search self
    pub async fn search(self) {}

    // install self
    pub async fn install(self) {}

    // uninstall self
    pub async fn uninstall(self) {}
}
