#![allow(dead_code)]
#![allow(unused_variables)]

use dash;

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

    let data = "
info:
    name: &name dash
    version: 0.1.0
    author: miten
    description: dash your way through OS post-install

system:
    os: Linux
    distro: Arch Linux
    pkg_mgr: &pkg_mgr pacman
    pkgs: 
        - discord
        - chrome
    dotfiles:
        - .zshrc
        - .vimrc
    run:
        - sudo *pkg_mgr -Syu
        - echo *name
    "
    .to_string();

    let parsed: serde_yaml::Value = serde_yaml::from_str(data.as_str()).unwrap();

    return;

    // loop {
    //     let start_byte = match parsed_string.find("join:") {
    //         Some(s) => s,
    //         None => break
    //     };

    //     let mut end_byte = start_byte + 1;
    //     for i in parsed_string[start_byte..].to_string().chars() {
    //         if i == ']' {
    //             break;
    //         }
    //         end_byte += 1;
    //     }

    //     let result = parsed_string[start_byte..end_byte]
    //         .to_string()
    //         .replace("join: [", "")
    //         .replace(",", "")
    //         .replace("]", "");

    //     parsed_string = parsed_string.replace(&parsed_string[start_byte..end_byte], parsed_string.as_str());
    // }

    println!("{}", parsed_string);

    // println!("{:#?}", parsed);

    // let mut parsed = data.parse::<toml::Value>().unwrap();

    // loop {
    //     let start_bytes = data.find("{").unwrap_or(0);
    //     let end_bytes = data.find("}").unwrap_or(0);

    //     if start_bytes == 0 || end_bytes == 0 {
    //         break;
    //     }

    //     let result = data[start_bytes..end_bytes + 1].to_string();
    //     let clean_result = &result[1..&result.len() - 1]
    //         .split('.')
    //         .collect::<Vec<&str>>();

    //     if !parsed[clean_result[0]][clean_result[1]].is_str() {
    //         data = data.replace(
    //             result.as_str(),
    //             "{0}.{1}"
    //                 .replace("{0}", clean_result[0])
    //                 .as_str()
    //                 .replace("{1}", clean_result[1])
    //                 .as_str(),
    //         );
    //     } else {
    //         data = data.replace(
    //             result.as_str(),
    //             parsed[clean_result[0]][clean_result[1]].as_str().unwrap(),
    //         );
    //     }
    // }

    // parsed = data.parse::<toml::Value>().unwrap();

    // if parsed.get("info").is_some() && parsed["info"].is_table()
    // //    || parsed.get("information").is_some() && parsed["information"].is_table()
    // {
    //     for (i, j) in info {
    //         if parsed["info"].get(i).is_none() {
    //             parsed["info"]["name"] = toml::Value::String(i.to_string());
    //         }
    //     }
    // }
}
