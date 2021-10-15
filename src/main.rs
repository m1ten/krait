#![allow(dead_code)]
#![allow(unused_variables)]

use dash;
use std::collections::HashMap;

fn main() {
    // let args = std::env::args().collect();

    // basic info about dash, stored into info
    let info = [
        ("name", "dash"),
        ("version", "0.1.0"),
        ("author", "miten"),
        ("description", "dash your way through OS post-install"),
    ];

    // get arguments are store into clap
    let clap = dash::Arguments::run(info);

    let mut data = "
info:
  is_dog: false
  occupation: null
    
setup:
  os: linux
  distro: Arch Linux
  pkg-mgr: pacman
  pkgs:
    - discord
    - firefox
    - chromium
  dotfiles:
    - .zshrc
    - .vimrc
    
scripts:
    run: \"sudo pacman -Syu\"
    before_pkgs: true
    run: \"rm -rf ~/.cache\"
    after_pkgs: true
    run: \"clear\"
    run: \"exit\"
        ".to_string();
    let data = {
        let i = data.matches("run:").count();
        let mut j = 1;
        while j != i + 1 {
            data = data.replacen("run:", "run_{}:".replace("{}", j.to_string().as_str()).as_str(), 1);
            j += 1;
        };
        data
    };

    println!("{:?}", data);

    return;

    let mut parsed: serde_yaml::Value = serde_yaml::from_str(data.as_str()).unwrap();
    println!("{:?}", parsed);

    for (i, j) in info {
        if parsed[i].is_null() {
            parsed[i] = serde_yaml::Value::String(j.to_string());
        }
    }

    // if !parsed["scripts"].is_null() {

    // }
}

// function to test config
fn test_config(vars: dash::Variables) {
    let chrome = dash::Package {
        name: String::from("Chrome"),
        prv_path: Some(String::from("~/Downloads/Chrome")),
        new_path: None,
        args: None,
    };

    let vimrc = dash::Dotfile {
        name: String::from(".vimrc"),
        prv_path: Some(String::from("~/Downloads/.vimrc")),
        new_path: Some(String::from("~/.vimrc")),
        symlink: Some(false),
    };

    let zshrc = dash::Dotfile {
        name: String::from(".zshrc"),
        prv_path: Some(String::from("~/Downloads/.zshrc")),
        new_path: Some(String::from("~/.zshrc")),
        symlink: Some(true),
    };

    let setup = dash::Setup {
        os: String::from("Windows"),
        distro: None,
        pkg_mgr: Some(String::from("winget")),
        pkg: vec![chrome],
        dotfile: vec![vimrc, zshrc],
    };

    let config = dash::Config {
        info: Some(vars),
        setup: Some(setup),
    };

    dash::Config::write(config);
}
