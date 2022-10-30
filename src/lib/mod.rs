pub mod args;
pub mod manifest;
pub mod pkg;
pub mod setup;
pub mod lua;

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use mlua::{DeserializeOptions, Lua, LuaSerdeExt, Table, Value};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

use crate as krait;

// read from file
pub fn readfs(path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// write to file
pub fn writefs(path: String, contents: String) -> Result<(), io::Error> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

// read from stdin
pub fn scan<T: std::str::FromStr>(stopper: u8) -> Result<T, ()> {
    let mut input = Vec::<u8>::new();

    let mut data: [u8; 1] = [0];
    loop {
        match std::io::stdin().read_exact(&mut data) {
            Ok(_) => {}
            Err(_) => return Err(()),
        }

        if data[0] != stopper && data[0] != '\n' as u8 {
            input.push(data[0]);
        } else {
            break;
        }
    }

    match std::str::from_utf8(&input).unwrap().trim().parse::<T>() {
        Ok(x) => Ok(x),
        Err(_) => Err(()),
    }
}

#[macro_export]
macro_rules! scan {
    ($str:tt, $_type:ty) => {{
        print!("{}", $str);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        krait::scan::<$_type>(' ' as u8).expect("scan failed")
    }};
}

#[macro_export]
macro_rules! scanln {
    ($str:tt) => {{
        print!("{}", $str);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        krait::scan::<String>('\n' as u8).expect("scanln failed")
    }};
}

// macro to clear console
#[macro_export]
macro_rules! clear {
    () => {{
        use std::process::Command;

        if cfg!(target_os = "windows") {
            Command::new("cmd").arg("/c").arg("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        }
    }};
}

#[macro_export]
macro_rules! exit {
    ($code: tt) => {{
        let key = if cfg!(target_os = "macos") {
            "return"
        } else {
            "enter"
        };
        let msg = format!("\nPress {} to exit.\n", key);
        krait::scanln!(msg);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! question {
    ($msg: tt) => {{
        loop {
            print!("{} [Y/n] ", $msg);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let answer = krait::scan::<String>('\n' as u8)
                .expect("question failed")
                .to_lowercase();
            if answer.trim() == "y" || answer.trim() == "yes" || answer.trim() == "" {
                break true;
            } else if answer.trim() == "n" || answer.trim() == "no" {
                break false;
            }
        }
    }};
}

#[macro_export]
macro_rules! kdbg {
    ($($x:tt)*) => {
        {
            #[cfg(debug_assertions)]
            {
                std::dbg!($($x)*)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($x)*)
            }
        }
    }
}

#[derive(SmartDefault, Serialize, Deserialize, Debug, Clone)]
pub struct KraitConfig {
    // krait name
    #[default(String::from("krait"))]
    pub name: String,

    // krait author
    #[default(String::from("miten <57693631+m1ten@users.noreply.github.com>"))]
    #[serde(default)]
    #[serde(alias = "maintainer")]
    pub author: String,

    // krait version
    #[default(String::from("0.0.1"))]
    #[serde(alias = "version")]
    pub ver: String,

    // krait description
    #[default(String::from("cross platform package manager"))]
    #[serde(default)]
    #[serde(alias = "description")]
    pub desc: String,

    // krait license
    #[default(String::from("Apache-2.0"))]
    pub license: String,

    // krait git repository
    #[default(String::from("https://github.com/m1ten/krait"))]
    pub git: String,

    #[default(None)]
    #[serde(alias = "packages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkgs: Option<Vec<String>>,

    #[default(dirs::home_dir().unwrap().join("krait"))]
    #[serde(default)]
    #[serde(alias = "directory")]
    pub dir: PathBuf,

    // krait package repository
    #[default(vec![String::from("https://github.com/m1ten/krait-pkgs")])]
    #[serde(alias = "repositories")]
    pub repos: Vec<String>,

    // krait default flags/args
    #[default(None)]
    #[serde(alias = "flags")]
    #[serde(alias = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl mlua::UserData for KraitConfig {}

impl KraitConfig {
    pub fn new() -> KraitConfig {
        KraitConfig::default()
    }

    pub fn gen_lua(&self) -> Vec<String> {

        let mut lua = mlua::Lua::new();
        let globals = lua.globals();

        let krait_t = lua.create_table().unwrap();
        let config_t = lua.create_table().unwrap();

        let mut dir = self.dir.clone().to_string_lossy().to_string();
        // check if running on windows
        if cfg!(target_os = "windows") {
            dir = dir.replace("\\", "\\\\");
        } else {
            dir = dir.replace("/", "\\/");
        }

        // assign values to config table
        config_t.set("name", self.name.clone()).unwrap();
        config_t.set("author", self.author.clone()).unwrap();
        config_t.set("ver", self.ver.clone()).unwrap();
        config_t.set("desc", self.desc.clone()).unwrap();
        config_t.set("license", self.license.clone()).unwrap();
        config_t.set("git", self.git.clone()).unwrap();
        config_t.set("pkgs", self.pkgs.clone()).unwrap();
        config_t.set("dir", dir).unwrap();
        config_t.set("args", self.args.clone()).unwrap();
        config_t.set("repos", self.repos.clone()).unwrap();

        // add config to krait table
        krait_t.set("config", config_t).unwrap();

        // add krait table to globals
        globals.set("krait", krait_t).unwrap();

        // get the krait table
        let krait_t = globals.get::<_, mlua::Table>("krait").unwrap();

        let result = lua::LuaState::gen_lua("krait".to_string(), krait_t);

        kdbg!(result.iter().map(|x| x.to_string()).collect::<String>());

        result
    }

    pub fn parse(config_str: String) -> KraitConfig {
        let lua = Lua::new();
        let globals = lua.globals();

        let krait_table = lua.create_table().expect("failed to create krait table");
        let config_table = lua.create_table().expect("failed to create config table");

        krait_table
            .set("config", config_table)
            .expect("failed to set config table");

        globals
            .set("krait", krait_table)
            .expect("failed to set krait table");

        // load the config
        lua.load(&config_str).exec().expect("failed to load config");

        // get the config as a table
        let krait_table: Table = globals.get("krait").expect("failed to get krait table");
        let config_table: Table = krait_table
            .get("config")
            .expect("failed to get config table");

        // deserialize the config table into a config struct using serde

        let options = DeserializeOptions::new()
            .deny_unsupported_types(false)
            .deny_recursive_tables(false);

        let krait_config: KraitConfig =
            match lua.from_value_with(Value::Table(config_table), options) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error parsing config: {}", e);
                    exit!(1);
                }
            };

        krait_config
    }
}