// Fields that should be added:
// - maintainer/contributor (type: string)

use std::{collections::HashMap, path::PathBuf};

use mlua::DeserializeOptions;
use mlua::LuaSerdeExt;
use mlua::Table;
use mlua::Value;
use regex::Regex;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::manifest::Manifest;
use crate::{self as krait};

use krait::exit;
use krait::kdbg;

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct Pkg {
    // name received from cli
    #[default("")]
    pub name: String,

    // version received from cli
    #[default("latest")]
    pub ver: String,

    #[default(None)]
    pub url: Option<String>,

    #[default(None)]
    pub path: Option<PathBuf>,

    #[default(None)]
    pub info_str: Option<String>,

    #[default(None)]
    pub info: Option<PkgInfo>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct PkgInfo {
    #[default(vec![String::new()])]
    #[serde(alias = "names")]
    pub name: Vec<String>,

    #[default(String::new())]
    #[serde(alias = "version")]
    pub ver: String,

    #[default(None)]
    #[serde(alias = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<Vec<String>>,

    #[default(None)]
    #[serde(alias = "package")]
    pub pkg: Option<Vec<PkgMain>>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct PkgMain {
    // any os that is posix AND/OR windows compliant
    #[default(None)]
    #[serde(alias = "support")]
    pub supports: Option<Vec<HashMap<String, Vec<String>>>>,

    // some programs have multiple repositories for each os
    #[default(None)]
    #[serde(alias = "repo", alias = "repository", alias = "repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos: Option<Vec<String>>,

    // some programs have multiple versions for each os
    #[default(None)]
    #[serde(alias = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,

    // dep and version
    #[default(None)]
    #[serde(alias = "dep", alias = "dependency", alias = "dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deps: Option<Vec<PkgDep>>,

    // whether the package is installed from git or binary
    #[default(String::from("binary"))]
    #[serde(rename = "type")]
    pub _type: String,

    // does not have to be code,
    // can be anything that will be included in the package
    // with direct links to the files
    // including hash of the file
    #[default(None)]
    #[serde(alias = "src", alias = "source", alias = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srcs: Option<Vec<PkgSrc>>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<Vec<PkgAction>>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install: Option<Vec<PkgAction>>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninstall: Option<Vec<PkgAction>>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<PkgAction>>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct PkgDep {
    #[default(String::new())]
    pub name: String,

    #[default(None)]
    pub ver: Option<String>,

    #[default(false)]
    pub dev: bool,

    #[default(false)]
    pub optional: bool,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct PkgSrc {
    // more than one url can be specified
    // order is important for mirrors
    // direct path required for binary
    #[serde(alias = "paths", alias = "url", alias = "link", alias = "links")]
    pub path: Vec<String>,

    // #[default(Some(false))]
    // #[serde(alias = "exec")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub exec: Option<bool>,

    // install and uninstall args
    // #[default(None)]
    // #[serde(alias = "arg", alias = "argument", alias = "arguments")]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub args: Option<HashMap<Vec<String>, Vec<String>>>,

    // hashes of the file (sha256, sha1, md5, etc)
    // e.g. (sha256: "...")
    // although not required, it is recommended to use
    #[default(None)]
    #[serde(alias = "hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<Vec<HashMap<String, String>>>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct PkgAction {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    // specific a condition
    #[default(None)]
    #[serde(rename = "if")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _if: Option<Vec<String>>,

    // use | to separate multiple commands
    #[default(None)]
    #[serde(alias = "cmd", alias = "cmds", alias = "command", alias = "commands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<String>,
}

impl Pkg {
    pub async fn fill(self, cache_dir: PathBuf, repos: Vec<String>) -> Self {
        kdbg!(repos.clone());

        // check if the package is already in the cache
        // create a new cache if it doesn't exist
        let cache = cache_dir.join(&self.name);

        kdbg!(&cache);

        // let mut version: Option<String> = None;

        // if cache.exists() && cache.is_dir() {

        //     // TODO: add support for cache


        //     // get pkg.lua from cache
        //     let pkg_lua_path = cache.join("pkg.lua");

        //     let pkg_lua_str = fs::read_to_string(pkg_lua_path)
        //         .await
        //         .map_err(|e| e.to_string())
        //         .expect("failed to read pkg.lua");

        //     let pkg_lua_ = PkgInfo::parse(pkg_lua_str.clone());

        //     kdbg!(&pkg_lua_);

        //     self.info_str = Some(pkg_lua_str.clone());
        //     self.info = Some(pkg_lua_);

        //     if self.ver.is_empty() || self.ver == "latest" {
        //         version = Some(self.info.expect("failed to get pkg.lua").ver);
        //     } else {
        //         // check if the version is valid
        //     }

        // }

        // if version is not specified, use the latest version

        let mut fail: bool = false;
        
        for repo in repos {
            // check if repo is valid 
            // if not, skip it

            // check if the repo is github link
            // if yes, use github api to get the latest version
            
            let lc = &repo.to_lowercase();

            let re =
                Regex::new(r"[a-z0-9]+://(?P<domain>[^/]+)/(?P<owner>[^/]+)/(?P<repo>[^/]+)/?")
                    .unwrap();

            let re_cap = match re.captures(lc) {
                Some(cap) => cap,
                None => {
                    fail = true;
                    continue;
                },
            };

            let domain = re_cap.name("domain").unwrap().as_str();

            if domain != "github.com" {
                // TODO: add support for non-github repos (e.g. gitlab, bitbucket)
                eprintln!("{domain} is currently not supported.");
                fail = true;
                continue;
            }

            let owner = re_cap.name("owner").unwrap().as_str();
            let repo = re_cap.name("repo").unwrap().as_str();

            dbg!(format!("Searching for {owner}/{repo}..."));

            // search for the package on github repo
            let api_url = format!(
                "https://api.github.com/repos/{owner}/{repo}/contents/manifest.lua",
            );

            dbg!(&api_url);

            // get the manifest.lua file and save it to the cache directory
            let manifest_json_str = match reqwest::get(&api_url).await {
                Ok(res) => match res.text().await {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Failed to get manifest.lua: {}", e);
                        fail = true;
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to get manifest.lua: {}", e);
                    fail = true;
                    continue;
                }
            };

            // parse the json string
            let manifest_json: serde_json::Value = match serde_json::from_str(&manifest_json_str) {
                Ok(json) => json,
                Err(e) => {
                    eprintln!("Failed to parse manifest.lua: {}", e);
                    fail = true;
                    continue;
                }
            };

            dbg!(&manifest_json);

            let down_url = match manifest_json["download_url"].as_str() {
                Some(url) => url,
                None => {
                    eprintln!("Failed to get download_url");
                    fail = true;
                    continue;
                }
            };

            // download the manifest.lua file and save it to the cache directory
            let manifest_lua_str = match reqwest::get(down_url).await {
                Ok(res) => match res.text().await {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Failed to get manifest.lua: {}", e);
                        fail = true;
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to get manifest.lua: {}", e);
                    fail = true;
                    continue;
                }
            };

            
            let manifest_lua = Manifest::parse(manifest_lua_str.clone());

            // TODO: work on this
            
        }

        if fail {
            eprintln!("Failed to find the package {}.", self.name);
            exit!(1);
        }

        self


    }
}

impl PkgInfo {
    pub fn parse(script_str: String) -> PkgInfo {
        let lua = mlua::Lua::new();
        let globals = lua.globals();

        let krait_table = lua.create_table().expect("failed to create krait table");
        let pkg_table = lua.create_table().expect("failed to create pkg table");

        krait_table
            .set("pkg", pkg_table)
            .expect("failed to set config table");

        globals
            .set("krait", krait_table)
            .expect("failed to set krait table");

        // load the pkg script
        let result = lua.load(&script_str).exec();

        if let Err(e) = result {
            eprintln!("Error parsing config: {}", e);
            exit!(1);
        }

        // get the script as a table
        let krait_table: Table = globals.get("krait").expect("failed to get krait table");
        let pkg_table: Table = krait_table.get("pkg").expect("failed to get pkg table");

        // deserialize the pkg table into a PkgInfo struct using serde

        let options = DeserializeOptions::new()
            .deny_unsupported_types(false)
            .deny_recursive_tables(false);

        let pkg_info: PkgInfo = match lua.from_value_with(Value::Table(pkg_table), options) {
            Ok(pkg_info) => pkg_info,
            Err(e) => {
                eprintln!("Error parsing config: {}", e);
                exit!(1);
            }
        };

        pkg_info
    }
}