// Fields that should be added:
// - maintainer/contributor (type: string)

use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use mlua::DeserializeOptions;
use mlua::LuaSerdeExt;
use mlua::Table;
use mlua::Value;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
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
    #[default(String::new())]
    pub name: String,

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
    pub async fn fill(mut self, cache_dir: &Path, repos: &Vec<String>) -> Self {
        kdbg!(&repos);

        // check if the package is already in the cache
        // create a new cache if it doesn't exist
        let cache = cache_dir.join(&self.name);

        kdbg!(&cache);

        // TODO: add support for cache

        let mut fail: bool = false;

        for repo in repos {
            if fail {
                fail = false;
            }

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
                }
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
            let api_url =
                format!("https://api.github.com/repos/{owner}/{repo}/contents/manifest.lua",);

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

            // write the manifest.lua file to the cache directory
            let manifest_lua_path = cache.join("manifest.lua");

            if let Err(e) = std::fs::write(&manifest_lua_path, &manifest_lua_str) {
                eprintln!("Failed to write manifest.lua: {}", e);
                fail = true;
                continue;
            }

            // verify the manifest.lua file hash
            let manifest_lua_hash = match manifest_json["sha"].as_str() {
                Some(hash) => hash,
                None => {
                    eprintln!("Failed to get manifest.lua hash");
                    fail = true;
                    continue;
                }
            };

            let mut hasher = Sha1::new();

            let mut file = match File::open(&manifest_lua_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open manifest.lua: {}", e);
                    fail = true;
                    continue;
                }
            };

            match std::io::copy(&mut file, &mut hasher) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("Error hashing file manifest.lua: {}", e);
                    krait::exit!(1);
                }
            };

            let hash_bytes = hasher.finalize();

            let hash = format!("{:x}", hash_bytes);

            if hash != *manifest_lua_hash {
                eprintln!("manifest.lua hash mismatch");
                fail = true;
                continue;
            }

            // let full_repo = format!("{owner}/{repo}", owner = owner, repo = repo);

            let manifest_lua = Manifest::parse(manifest_lua_str.clone());

            let mut ver_commit: Option<String> = None;

            for pkg in manifest_lua.packages {
                if pkg.0 == self.name {
                    for pkg in pkg.1 {
                        if pkg.0 == self.ver {
                            // get the latest ManifestPackage from pkg.1
                            let pkg = match pkg.1.last() {
                                Some(pkg) => pkg,
                                None => {
                                    eprintln!("Failed to get ManifestPackage");
                                    fail = true;
                                    continue;
                                }
                            };

                            ver_commit = Some(pkg.commit.clone());

                            // download everything from pkg.contents and put it into root/packages/{pkg_name}

                            let mut sha1_url: HashMap<String, String> = HashMap::new();

                            for c in pkg.contents.clone() {
                                sha1_url.insert(c.sha1, c.url);
                            }

                            dbg!(&sha1_url);

                            // download the files
                            for (sha1, url) in sha1_url {
                                // write to /cache/packages/{pkg_name}/{sha1}
                                let cache_path = cache
                                    .join(self.name.clone())
                                    .join(ver_commit.clone().unwrap());

                                // check if the file exists, if yes, overwrite it
                                if cache_path.exists() {
                                    std::fs::remove_file(&cache_path)
                                        .map_err(|e| e.to_string())
                                        .expect("failed to remove file");
                                }

                                let mut file = std::fs::File::create(&cache_path)
                                    .map_err(|e| e.to_string())
                                    .expect("failed to create file");

                                let res = match reqwest::get(url)
                                    .await
                                    .map_err(|e| e.to_string())
                                    .expect("failed to get file")
                                    .bytes()
                                    .await
                                {
                                    Ok(res) => res,
                                    Err(e) => {
                                        eprintln!("Failed to get file: {}", e);
                                        fail = true;
                                        continue;
                                    }
                                };

                                // write the file
                                match file.write_all(&res) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprintln!("Failed to write file: {}", e);
                                        fail = true;
                                        continue;
                                    }
                                }

                                // check the sha1 of the file
                                let mut hasher = Sha1::new();
                                let mut file = std::fs::File::open(&cache_path)
                                    .map_err(|e| e.to_string())
                                    .expect("failed to open file");

                                match std::io::copy(&mut file, &mut hasher) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        eprintln!("Failed to hash file: {}", e);
                                        fail = true;
                                        continue;
                                    }
                                }

                                let hash = format!("{:x}", hasher.finalize());

                                if hash != sha1 {
                                    eprintln!("sha1 of file does not match");
                                    fail = true;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }

            // get the package script from cache/packages/{pkg_name}/{sha1}/manifest.lua

            if ver_commit.is_none() {
                eprintln!("Failed to get commit hash");
                fail = true;
                continue;
            }

            let package_path = cache
                .join(self.name.clone())
                .join(ver_commit.clone().unwrap());

            let package_manifest_lua_path = package_path.join("manifest.lua");

            let package_manifest_lua_str = match std::fs::read_to_string(package_manifest_lua_path)
            {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Failed to read package manifest.lua: {}", e);
                    fail = true;
                    continue;
                }
            };

            let package_manifest_lua = PkgInfo::parse(package_manifest_lua_str.clone());

            let package_url = format!(
                "https://api.github.com/repos/{}/{}/contents/src/lib?ref={}",
                owner,
                repo,
                ver_commit.clone().unwrap()
            );

            self.url = Some(package_url);
            self.path = Some(package_path);
            self.info_str = Some(package_manifest_lua_str);
            self.info = Some(package_manifest_lua);
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
            kdbg!(e.clone());
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
