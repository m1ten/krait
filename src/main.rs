#![allow(dead_code)]
#![allow(unused_variables)]

use dash;

fn main() {
    // let args = std::env::args().collect();

    // basic info about dash, stored into vars
    let vars = dash::Variables {
        name: String::from("dash"),
        version: String::from("0.1.0"),
        author: String::from("miten"),
        description: String::from("dash your way through OS post-install"),
    };

    // get arguments are store into clap
    let clap = dash::Arguments::run(vars.clone());

    // let c_data = dash::Config::read();

    // if c_data.is_none() {
    //     println!("None = {:?}", c_data);
    // } else {
    //     println!("Some = {:?}", c_data);
    // }

    // test_config(vars);
    let data = "
    info:
        name: dash
        version: 0.1.0
        is_dog: false
        occupation: null
        author: miten
        description: dash your way through OS post-install
        ";

    let docs = yaml_rust::YamlLoader::load_from_str(data).unwrap();
    let doc = &docs[0];

    println!("{:?}", doc["info"]["name"].as_str().unwrap());

    #[derive(Debug, Clone)] 
    enum CEnum {
        Str(String),
        Int(i64),
        Bool(bool),
        Null(null)
    }


    let mut data = [
        CEnum::Str("name".to_string()),
        CEnum::Str("version".to_string()),
        CEnum::Str("is_dog".to_string()),
        CEnum::Str("occupation".to_string()),
        CEnum::Str("author".to_string()),
        CEnum::Str("description".to_string()),
    ];

    let mut l = 0;
    for j in data.clone() {
        match j {
            CEnum::Str(o) => {
                match &doc["info"][o.as_str()] {
                    yaml_rust::yaml::Yaml::String(s) => data[l] = CEnum::Str(s),
                    yaml_rust::yaml::Yaml::Boolean(b) => data[l] = CEnum::Bool(*b),
                    yaml_rust::yaml::Yaml::Integer(i) => data[l] = CEnum::Int(*i),
                    _ => (),
                }
            },
            _ => ()
        }
        l += 1;
    }

    println!("{:?}", data);
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
