pub mod args;
pub mod pkg;
pub mod setup;

use std::{
    fs::File,
    io::{self, Read, Write},
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
        wix::scan::<$_type>(' ' as u8).expect("scan failed")
    }};
}

#[macro_export]
macro_rules! scanln {
    ($str:tt) => {{
        print!("{}", $str);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        wix::scan::<String>('\n' as u8).expect("scanln failed")
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
        wix::scanln!(msg);
        std::process::exit($code);
    }};
}

#[macro_export]
macro_rules! question {
    ($msg: tt) => {{
        loop {
            print!("{} [Y/n] ", $msg);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let answer = wix::scan::<String>('\n' as u8)
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
pub struct WixConfig {
    pub gen: WixGen,
    pub pkg: WixPkg,
    pub dir: WixDir,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WixGen {
    // wix name
    pub name: String,

    // wix author
    #[serde(default)]
    pub author: String,

    // wix version
    pub ver: String,

    // wix description
    #[serde(default)]
    pub desc: String,

    // wix license
    pub license: String,

    // wix git repository
    pub git: String,

    // wix repository
    pub repos: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WixPkg {
    // installed pkgs
    pub pkgs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WixDir {
    // wix directory for posix

    #[serde(default)]
    pub dir_posix: String,

    #[serde(default)]
    pub bin_dir_posix: String,

    #[serde(default)]
    pub cache_dir_posix: String,

    #[serde(default)]
    pub temp_dir_posix: String,

    // wix directory for windows

    #[serde(default)]
    pub dir_windows: String,

    #[serde(default)]
    pub bin_dir_windows: String,

    #[serde(default)]
    pub cache_dir_windows: String,

    #[serde(default)]
    pub temp_dir_windows: String,
}

// set default values for config
impl Default for WixConfig {
    fn default() -> Self {
        WixConfig {
            gen: WixGen {
                name: "wix".to_string(),
                author: "miten".to_string(),
                ver: "0.1.0".to_string(),
                desc: "cross platform package manager".to_string(),
                license: "zlib".to_string(),
                git: "https://github.com/m1ten/wix".to_string(),
                repos: vec!["https://github.com/m1ten/wix-pkgs/".to_string()],
            },
            pkg: WixPkg { pkgs: vec![] },
            dir: WixDir {
                dir_posix: "~/wix".to_string(),
                bin_dir_posix: "~/wix/bin".to_string(),
                cache_dir_posix: "~/wix/cache".to_string(),
                temp_dir_posix: "~/wix/temp".to_string(),
                dir_windows: "~/wix".to_string(),
                bin_dir_windows: "~/wix/bin".to_string(),
                cache_dir_windows: "~/wix/cache".to_string(),
                temp_dir_windows: "~/wix/temp".to_string(),
            },
        }
    }
}
