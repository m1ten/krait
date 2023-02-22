use mlua::UserData;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::scripts::KraitScript;

use self::config::KraitConfig;
use self::manifest::RepoManifest;
use self::pkg::PkgManifest;

pub mod config;
pub mod manifest;
pub mod pkg;

#[derive(Deserialize, Serialize, Debug, Clone)]
enum ArgCommand {
    Install(bool, Vec<String>),
    Uninstall(bool, Vec<String>),
    Update(bool, Vec<String>),
    Clean(bool),
    Help(bool),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum ArgOption {
    Verbose(bool),
    Debug(bool),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
enum ScriptArgs {
    /// arguments passed to the script
    /// TODO: implement script args
    TODO,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct KraitCli {
    /// arguments passed to the script
    /// TODO: implement script args
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_args: Option<ScriptArgs>,

    /// arguments passed to krait
    #[default((ArgCommand::Help(true), None))]
    pub krait_args: (ArgCommand, Option<Vec<ArgOption>>),
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct KraitStd {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<KraitConfig>,

    /// arguments passed to the script
    /// TODO: implement script args
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_args: Option<ScriptArgs>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg_manifest: Option<PkgManifest>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_manifest: Option<RepoManifest>,

    /// built-in functions and variables will be defined here ~~later~~
}

/// Temporary type alias for testing
pub type KraitMain = KraitStd;

// #[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
// pub struct KraitMain {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub config: Option<KraitConfig>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pkgs: Option<Vec<Pkg>>,

//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub manifest: Option<KraitManifest>,
// }

impl UserData for KraitMain {}
impl KraitScript for KraitMain {}
