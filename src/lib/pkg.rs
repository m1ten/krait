// Fields that should be added:
// - maintainer/contributor (type: string)

use std::{collections::HashMap, path::PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use tokio::fs;

use crate::{self as wix};

use wix::{dbg, exit};

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
    pub info_yml: Option<PkgInfo>,
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
    pub deps: Option<Vec<HashMap<String, String>>>,

    // dep and version
    #[default(None)]
    #[serde(
        alias = "dev_dep",
        alias = "dev_dependency",
        alias = "dev_dependencies"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_deps: Option<Vec<HashMap<String, String>>>,

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
    pub async fn fill(self, cache_dir: PathBuf, repos: Vec<String>) -> Result<Self, String> {
        // check if the package is already in the cache
        // create folder for the package
        let cache = cache_dir.join(&self.name);

        dbg!(&cache);

        if cache.exists() && cache.is_dir() {
            // get pkg.yml from cache
            let pkg_yml = cache.join("pkg.yml");

            // read pkg.yml
            let pkg_yml = match fs::read_to_string(&pkg_yml).await {
                Ok(s) => match serde_yaml::from_str(&s) {
                    Ok(y) => (s, y),
                    Err(_) => ("err".to_string(), PkgInfo::default()),
                },
                Err(_) => ("err".to_string(), PkgInfo::default()),
            };

            let ver: String;
            // check if self.ver is latest
            if self.ver == "latest" {
                ver = pkg_yml.1.ver.clone();
            } else {
                ver = self.ver.clone();
            }

            // check if self.ver matches pkg.yml.ver
            if ver == pkg_yml.1.ver {
                let pkg = Pkg {
                    name: self.name,
                    ver: self.ver,
                    url: None,
                    path: Some(cache),
                    info_str: Some(pkg_yml.0),
                    info_yml: Some(pkg_yml.1),
                };

                return Ok(pkg);
            } else {
                // if not, delete the cache folder
                let _ = match fs::remove_dir_all(&cache).await {
                    Ok(_) => (),
                    Err(_) => return Err("Could not delete cache folder!".to_string()),
                };
            }
        }

        // search for the package on github repo
        // (domain, owner, repo); might add branch support later
        let mut vec_3: Vec<(String, String, String)> = Vec::new();

        for repo in repos {
            let lc = &repo.to_lowercase();
            let re =
                Regex::new(r"[a-z0-9]+://(?P<domain>[^/]+)/(?P<owner>[^/]+)/(?P<repo>[^/]+)/?")
                    .unwrap();

            let re_cap = match re.captures(lc) {
                Some(c) => c,
                None => continue,
            };

            let domain = re_cap.name("domain").unwrap().as_str();

            if domain != "github.com" {
                // TODO: add support for non-github repos (e.g. gitlab, bitbucket)
                eprintln!("{domain} is currently not supported.");
                continue; 
            }

            let owner = re_cap.name("owner").unwrap().as_str();
            let repo = re_cap.name("repo").unwrap().as_str();

            vec_3.push((domain.to_string(), owner.to_string(), repo.to_string()));
        }

        if vec_3.is_empty() {
            println!("No github repositories found.");
            exit!(1);
        }

        let owner = &vec_3[0].1;
        let repo = &vec_3[0].2;

        dbg!(format!("Searching for {owner}/{repo}."));

        // search for the package on github repo
        let api_url = format!(
            "https://api.github.com/repos/{owner}/{repo}/contents/manifests/{name}",
            name = self.name
        );

        // download the folder
        let client = reqwest::Client::new();
        let json = match client
            .get(&api_url)
            .header(reqwest::header::USER_AGENT, "wix")
            .send()
            .await
        {
            Ok(r) => match r.json::<serde_yaml::Value>().await {
                Ok(j) => j,
                Err(e) => return Err(format!("{}", e)),
            },
            Err(e) => return Err(format!("{}", e)),
        };

        let mut i = 0;
        // name and download_url
        let mut files: Vec<HashMap<String, String>> = Vec::new();
        loop {
            let name = match json[i]["name"].as_str() {
                Some(n) => (n),
                None => break,
            };

            let down = match json[i]["download_url"].as_str() {
                Some(d) => (d),
                None => break,
            };

            let mut hashmap = HashMap::new();
            hashmap.insert(name.to_string(), down.to_string());

            files.push(hashmap);

            i += 1;
        }

        dbg!(format!("Found {} files.", files.len()));

        let mut info_str: Option<String> = None;
        let mut info_yml: Option<PkgInfo> = None;
        let mut down_url: Option<String> = None;

        for f in &files {
            let ext = f.keys().next().unwrap();
            let url = f.values().next().unwrap();

            dbg!(format!("Downloading {url}."));
            let content = match client
                .get(url)
                .header(reqwest::header::USER_AGENT, "wix")
                .send()
                .await
            {
                Ok(r) => match r.text().await {
                    Ok(c) => c,
                    Err(e) => return Err(format!("{}", e)),
                },
                Err(e) => return Err(format!("{}", e)),
            };

            if ext.to_string() == format!("{}.yml", self.name) {
                info_str = Some(content.clone());
                info_yml = match serde_yaml::from_str(&info_str.as_ref().unwrap()) {
                    Ok(i) => Some(i),
                    Err(e) => return Err(format!("{}", e)),
                };
                down_url = Some(url.to_string());
            }

            // write to file
            dbg!(format!("Writing {ext}."));
            let cache_str = match cache.join(ext).to_str() {
                Some(c) => c.to_string(),
                None => return Err(format!("Invalid path.")),
            };
            dbg!(format!("{cache_str}"));

            // create folder if not exists
            if !cache.exists() {
                dbg!(format!("Creating folder {}.", cache.display()));
                let _ = match fs::create_dir_all(&cache).await {
                    Ok(_) => (),
                    Err(e) => return Err(format!("{}", e)),
                };
            }

            let _ = match wix::writefs(cache_str, content) {
                Ok(_) => (),
                Err(e) => return Err(format!("{}", e)),
            };
        }

        if info_str.is_none() || info_yml.is_none() {
            return Err(format!("No info.yml found."));
        }

        dbg!(format!("Downloaded {} files.", files.len()));

        let pkg = Pkg {
            name: self.name,
            ver: self.ver,
            url: Some(down_url.unwrap()),
            path: Some(cache),
            info_str: Some(info_str.unwrap()),
            info_yml: Some(info_yml.unwrap()),
        };

        Ok(pkg)
    }
}
