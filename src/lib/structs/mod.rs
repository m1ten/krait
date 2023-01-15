use mlua::UserData;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate::scripts::KraitScript;

use self::config::KraitConfig;
use self::manifest::KraitManifest;
use self::pkg::Pkg;

pub mod config;
pub mod manifest;
pub mod pkg;

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
