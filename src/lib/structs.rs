use indexmap::IndexMap;
use crate as wix;

#[derive(Debug, Clone)]
pub struct Information {
    // wix name
    pub name: String,

    // wix author
    pub author: String,

    // wix version
    pub version: String,

    // wix description
    pub description: String,

    // wix license
    pub license: String,

    // wix git repository
    pub git: String,
}

impl Information {
    pub fn get_field_type(info: Option<Information>) -> IndexMap<String, String> {
        let info = match info {
            Some(i) => i,
            None => {
                let mut map = IndexMap::new();
                map.insert("name".to_string(), "String".to_string());
                map.insert("author".to_string(), "String".to_string());
                map.insert("version".to_string(), "String".to_string());
                map.insert("description".to_string(), "String".to_string());
                map.insert("license".to_string(), "String".to_string());
                map.insert("git".to_string(), "String".to_string());
                return map;
            }
        };
        let mut map = IndexMap::new();
        map.insert("name".to_string(), info.name.clone());
        map.insert("author".to_string(), info.author.clone());
        map.insert("version".to_string(), info.version.clone());
        map.insert("description".to_string(), info.description.clone());
        map.insert("license".to_string(), info.license.clone());
        map.insert("git".to_string(), info.git.clone());
        map
    }
}

#[derive(Debug, Clone)]
pub struct Configuration {
    // wix repo
    pub repo: String,

    // wix name
    pub mirror: Option<String>,
}

impl Configuration {
    pub fn get_field_type(info: Option<Configuration>) -> IndexMap<String, String> {
        let info = match info {
            Some(i) => i,
            None => {
                let mut map = IndexMap::new();
                map.insert("repo".to_string(), "String".to_string());
                map.insert("mirror".to_string(), "String".to_string());
                return map;
            }
        };
        let mut map = IndexMap::new();
        map.insert("repo".to_string(), info.repo.clone());
        map.insert("mirror".to_string(), info.mirror.clone().unwrap_or("".to_string()));
        map
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
   pub fn install(script: String, name: String, path: String) {
       // TODO: check if package is already installed

       println!("\nReview Script\n{}", script);

       let question = format!("\nDo you want to install {}?", name);

       if wix::question!(question) {
            println!("Installing {}.", name);            

       }
   }
}