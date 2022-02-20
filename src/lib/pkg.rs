// Fields that should be added:
// - maintainer/contributor (type: string)

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

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
    pub supports: Option<HashMap<String, Vec<String>>>,

    // some programs have multiple repositories for each os
    #[default(None)]
    #[serde(alias = "repo", alias = "repository", alias = "repositories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos: Option<Vec<String>>,

    #[default(None)]
    #[serde(alias = "dep", alias = "dependency", alias = "dependencies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deps: Option<Vec<String>>,

    #[default(None)]
    #[serde(
        alias = "dev_dep",
        alias = "dev_dependency",
        alias = "dev_dependencies"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dev_deps: Option<Vec<String>>,

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
    pub install: Option<Vec<String>>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uninstall: Option<Vec<String>>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct PkgSrc {
    // more than one url can be specified
    // order is important for mirrors
    #[serde(alias = "url", alias = "link", alias = "links")]
    pub urls: Vec<String>,

    // hashes of the file (sha256, sha1, md5, etc)
    // e.g. (sha256: "...")
    // although not required, it is recommended to use
    #[default(None)]
    #[serde(alias = "hash", alias = "hashes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<HashMap<String, String>>,
}