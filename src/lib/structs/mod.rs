use mlua::UserData;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::scripts::KraitScript;

use self::config::KraitConfig;
use self::manifest::KraitManifest;
use self::pkg::{Pkg, PkgData};

pub mod config;
pub mod manifest;
pub mod pkg;

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct KraitCli {
    /// implement this later
}


#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct KraitStd {
    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<KraitConfig>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli: Option<KraitCli>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkg: Option<PkgData>,

    #[default(None)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<KraitManifest>,
}

#[derive(SmartDefault, Deserialize, Serialize, Debug, Clone)]
pub struct KraitMain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<KraitConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgs: Option<Vec<Pkg>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<KraitManifest>,
}

impl UserData for KraitMain {}
impl KraitScript for KraitMain {}