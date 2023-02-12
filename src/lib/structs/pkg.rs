// Fields that should be added:
// - maintainer/contributor (type: string)

use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use mlua::DeserializeOptions;
use mlua::LuaSerdeExt;
use mlua::Table;
use mlua::Value;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::exit;
use crate::kdbg;

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
/// Items received from cli
pub struct PkgCli {
    /// Required: Krait is not magic
    #[default(String::new())] 
    pub name: String,

    /// Optional: defaults to latest (not yet implemented)
    #[default(String::from("latest"))]
    pub ver: String,

    /// Path to package
    #[default(None)]
    pub path: Option<PathBuf>,

    /// Other fields I am probably missing before data
    /// This data is actual serialized data from '{pkg}/main.lua'
    pub data: Option<PkgData>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
/// Items received from '{pkg}/main.lua'
/// This is the data that is actually used to install the package
pub struct PkgData {
    /// Required: Krait is not magic
    /// First name should be the folder name 
    #[serde(alias = "name")]
    pub names: Vec<String>,

    /// Description of package
    #[default(None)]
    #[serde(alias = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,

    /// License of package (can be multiple)
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<Vec<String>>,   

    #[default(None)]
    // TODO: left off here 
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

// impl Pkg {
//     pub async fn fill(self, cache_dir: &Path, repos: &Vec<String>) -> Self {
//         kdbg!(self);
//         kdbg!(cache_dir);
//         kdbg!(repos);

//         todo!("fill pkg info");
//     }
// }

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
