#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

mod lib;

fn main() {
    
}

//     return;

//     let strtoml = "
    
// [info]
// name = 'dash'
// version = '0.1.0'
// author = 'none'
// description = 'dash your way through OS post-install'

// [system]
// os = 'Linux'
// distro = 'Arch Linux'
// pkg_mgr = 'pacman'
// pkgs = [ 'discord', 'chrome' ]
// dotfiles = [ '.zshrc', '.vimrc' ]

// [[scripts]]
// run = '*pkg_mgr -Syu'
// with = 'root'
// without = 'verbose'

// [[scripts]]
// run = 'echo *name'
// with = 'verbose'
// without = 'root'
    
//     ".to_string();

//     return;

//     // let args = std::env::args().collect();

//     // basic info about dash, stored into info
//     let info = [
//         ("name", "dash"),
//         ("version", "0.1.0"),
//         ("author", "miten"),
//         ("description", "dash your way through OS post-install"),
//     ];

//     // get arguments are store into clap
//     let clap = lib::Arguments::run(info);

//     let mut data = "

// info:
//     name: &name dash
//     version: 0.1.0
//     author: miten
//     description: dash your way through OS post-install

// system:
//     os: Linux
//     distro: Arch Linux
//     pkg_mgr: &pkg_mgr pacman
//     pkgs: 
//         - discord
//         - chrome
//     dotfiles:
//         - .zshrc
//         - .vimrc

// scripts:
//     - run: *pkg_mgr -Syu
//       with: root
//       without: verbose
//     - run: echo *name 
//       with: verbose
//       without: root

//     "
//     .to_string();

//     loop {
//         let asterisk_start_byte = match data.find("*") {
//             Some(s) => s,
//             None => break
//         };
//         let mut asterisk_end_byte = asterisk_start_byte;

//         for i in data[asterisk_start_byte..].to_string().chars() {
//             if i != ' ' {
//                 asterisk_end_byte += 1;
//             } else {
//                 break;
//             }
//         }

//         let asterisk = &data[asterisk_start_byte..asterisk_end_byte];

//         let start_byte = match data.find(&"&{}".replace("{}", &asterisk.replace("*", ""))) {
//             Some(s) => s,
//             None => continue
//         };
//         let mut middle_byte = start_byte + 1;
//         let mut end_byte = start_byte + 1;

//         for i in data[start_byte..].to_string().chars() {
//             if i != ' ' {
//                 middle_byte += 1;
//             } else {
//                 break;
//             }
//         }
        
//         for i in data[start_byte..].to_string().chars() {
//             if i != '\n' {
//                 end_byte += 1;
//             } else {
//                 break;
//             }
//         }

//         // let ampersand = &data[start_byte..middle_byte];
//         let result = &data[middle_byte..end_byte];

//         data = data.replace(asterisk, &result.replace("\n", ""));
//     }

//     println!("{}", data);

//     let parsed = match serde_yaml::from_str::<serde_yaml::Value>(data.as_str()) {
//         Ok(k) => k,
//         Err(e) => serde_yaml::Value::String(e.to_string())
//     };
// }