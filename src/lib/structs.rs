use std::collections::HashMap;

use crate::{self as wix, exit};
use indexmap::IndexMap;

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
    pub fn get_field_type(config: Option<Configuration>) -> IndexMap<String, String> {
        let config = match config {
            Some(i) => i,
            None => {
                let mut map = IndexMap::new();
                map.insert("repo".to_string(), "String".to_string());
                map.insert("mirror".to_string(), "String".to_string());
                return map;
            }
        };
        let mut map = IndexMap::new();
        map.insert("repo".to_string(), config.repo.clone());
        map.insert(
            "mirror".to_string(),
            config.mirror.clone().unwrap_or("".to_string()),
        );
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
    // function to install package
    pub fn install(script: String, name: String, path: String) {
        println!("\nReview Script\n{}", script);

        let question = format!("Do you want to install {}?", name);

        if wix::question!(question) {
            println!("\nInstalling {}.\n", name);

            let function = wix::lang::get_data::<bool>(
                script.clone(),
                path.clone(),
                name.clone(),
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
            let mut installed_packages = wix::lang::get_data::<HashMap<String, String>>(
                config.clone(),
                wix_path.clone(),
                "wix".to_string(),
                Some("installed_packages".to_string()),
                None,
            )
            .unwrap();

            let version = "latest".to_string();

            // check if package is already installed
            if installed_packages.contains_key(&name) {
                println!("{} is already installed.", name);
                exit!(1);
            }

            if function.unwrap_err().contains("TypeError: 'function'") {

                // call install function
                match wix::lang::call_func(script, path, name.clone(), "install".to_string()) {
                    Ok(()) => println!("\n{} installed successfully.", name),
                    Err(e) => {
                        println!("\n{} failed to install.", name);
                        println!("{}", e);
                        exit!(1);
                    }
                }

                
            }

            // add package to installed_packages list
            installed_packages.insert(name.clone(), version.clone());

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
    pub fn uninstall(script: String, name: String, path: String) {
        println!("\nReview Script\n{}", script);

        let question = format!("Do you want to uninstall {}?", name);

        if wix::question!(question) {
            println!("\nUninstalling {}.\n", name);

            let function = wix::lang::get_data::<bool>(
                script.clone(),
                path.clone(),
                name.clone(),
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
            let mut installed_packages = wix::lang::get_data::<HashMap<String, String>>(
                config.clone(),
                wix_path.clone(),
                "wix".to_string(),
                Some("installed_packages".to_string()),
                None,
            )
            .unwrap();

            // check if package is already installed
            if !installed_packages.contains_key(&name) {
                println!("{} is not installed.", name);
                exit!(1);
            }

            if function.unwrap_err().contains("TypeError: 'function'") {

                // call install function
                match wix::lang::call_func(script, path, name.clone(), "uninstall".to_string()) {
                    Ok(()) => println!("\n{} uninstalled successfully.", name),
                    Err(e) => {
                        println!("\n{} failed to uninstall.", name);
                        println!("{}", e);
                        exit!(1);
                    }
                }
                
            }

            // remove package from installed_packages list
            installed_packages.remove(&name);

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