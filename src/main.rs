use dash;

fn main() {
    // let args = std::env::args().collect();

    let vars = dash::Variables {
        name: String::from("dash"),
        version: String::from("0.1.0"),
        author: String::from("miten"),
        description: String::from("dash your way through OS post-install"),
    };

    let clap = dash::Arguments::run(vars.clone());

    // test_config(vars);
}

/*
fn test_config(vars: dash::Variables) {
    let chrome = dash::Pkgfile {
        name: String::from("Chrome"),
        prv_path: Some(String::from("~/Downloads/Chrome")),
        new_path: None
    };

    let vimrc = dash::Pkgfile {
        name: String::from(".vimrc"),
        prv_path: Some(String::from("~/Downloads/.vimrc")),
        new_path: Some(String::from("~/.vimrc"))
    };

    let zshrc = dash::Pkgfile {
        name: String::from(".zshrc"),
        prv_path: Some(String::from("~/Downloads/.zshrc")),
        new_path: Some(String::from("~/.zshrc"))
    };

    let setup = dash::Setup {
        os: String::from("Windows"),
        distro: None,
        pkg_mgr: Some(String::from("winget")),
        pkg: vec![chrome],
        dotfile: vec![vimrc, zshrc]
    };

    let config = dash::Config {
        info: Some(vars),
        setup: Some(setup)
    };

    dash::Config::write(config);
}
*/