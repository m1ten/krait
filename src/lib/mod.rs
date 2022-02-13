pub mod args;
pub mod pkg;
pub mod setup;

use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

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
        neo::scan::<$_type>(' ' as u8).expect("scan failed")
    }};
}

#[macro_export]
macro_rules! scanln {
    ($str:tt) => {{
        print!("{}", $str);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        neo::scan::<String>('\n' as u8).expect("scanln failed")
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
        neo::scanln!(msg);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! question {
    ($msg: tt) => {{
        loop {
            print!("{} [Y/n] ", $msg);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let answer = neo::scan::<String>('\n' as u8)
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeoConfig {
    pub gen: NeoGen,
    pub pkg: NeoPkg,
    pub dir: NeoDir,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeoGen {
    // neo name
    pub name: String,

    // neo author
    #[serde(default)]
    pub author: String,

    // neo version
    pub ver: String,

    // neo description
    #[serde(default)]
    pub desc: String,

    // neo license
    pub license: String,

    // neo git repository
    pub git: String,

    // neo repository
    pub repos: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeoPkg {
    // installed pkgs
    pub pkgs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NeoDir {
    // neo directory for posix
    #[serde(default)]
    pub dir: PathBuf,

    #[serde(default)]
    pub bin_dir: PathBuf,

    #[serde(default)]
    pub cache_dir: PathBuf,

    #[serde(default)]
    pub temp_dir: PathBuf,
}

// set default values for config
impl Default for NeoConfig {
    fn default() -> Self {
        NeoConfig {
            gen: NeoGen {
                name: "neo".to_string(),
                author: "miten".to_string(),
                ver: "0.1.0".to_string(),
                desc: "cross platform package manager".to_string(),
                license: "zlib".to_string(),
                git: "https://github.com/m1ten/neopkg".to_string(),
                repos: vec!["https://github.com/m1ten/neo-pkgs".to_string()],
            },
            pkg: NeoPkg { pkgs: vec![] },
            dir: NeoDir {
                dir: "~/neopkg".into(),
                bin_dir: "~/neopkg/bin".into(),
                cache_dir: "~/neopkg/cache".into(),
                temp_dir: "~/neopkg/temp".into(),
            },
        }
    }
}
