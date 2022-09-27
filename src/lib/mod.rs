pub mod args;
pub mod pkg;
pub mod setup;

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use mlua::{DeserializeOptions, LuaSerdeExt, Table, Value, Lua};
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
    pub author: String,

    // krait version
    #[default(String::from("0.0.1"))]
    #[serde(alias = "version")]
    pub ver: String,

    // krait description
    #[default(String::from("cross platform package manager"))]
    #[serde(default)]
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
    pub dir: PathBuf,

    // krait package repository
    #[default(vec![String::from("https://github.com/m1ten/krait-pkgs")])]
    pub repos: Vec<String>,

    // krait default flags/args
    #[default(None)]
    #[serde(alias = "flags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
}

impl mlua::UserData for KraitConfig {}

impl KraitConfig {
    pub fn new() -> KraitConfig {
        KraitConfig::default()
    }

    pub fn gen_lua(&self) -> String {
        let carg = match self.args {
            Some(ref x) => format!("{{ \"{}\" }}", x.join("\", \"")),
            None => String::from("{}"),
        };

        let cpkg = match self.pkgs {
            Some(ref x) => format!("{{ \"{}\" }}", x.join("\", \"")),
            None => String::from("{}"),
        };

        let crepo = format!("{{ \"{}\" }}", self.repos.join("\", \""));

        let mut dir = self.dir.clone().to_string_lossy().to_string();
        // check if running on windows
        if cfg!(target_os = "windows") {
            dir = dir.replace("\\", "\\\\");
        } else {
            dir = dir.replace("/", "\\/");
        }

        let mut string = format!(
            "
--           Krait Config           --
-- Automatically generated by Krait --
--      READ THE DOCUMENTATION      --

local c = krait.config

c.name = \"{}\"
c.author = \"{}\"
c.ver = \"{}\"
c.desc = \"{}\"
c.license = \"{}\"
c.git = \"{}\"
c.pkgs = {}

-- Feel free to modify the following lines --

c.dir = \"{}\"
c.repos = {}
c.args = {}
            ",
            self.name,
            self.author,
            self.ver,
            self.desc,
            self.license,
            self.git,
            cpkg,
            dir,
            crepo,
            carg,
        );

        // remove the tabs
        string = string.replace("\t", "");

        string
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
        let result = lua.load(&config_str).exec();

        if let Err(e) = result {
            eprintln!("Error parsing config: {}", e);
            exit!(1);
        }

        // get the config as a table
        let krait_table: Table = globals.get("krait").expect("failed to get krait table");
        let config_table: Table = krait_table.get("config").expect("failed to get config table");

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