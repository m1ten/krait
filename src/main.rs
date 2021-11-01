#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

fn main() {
	let info = wix::Information {
		name: "wix".to_string(),
		author: "miten".to_string(),
		version: "0.1.0".to_string(),
		description: "wix - cross platform package manager".to_string(),
	};

	let args = wix::args::Arguments::new(info);

	println!("{:?}", args);

	let file = "discord.py";
	let code = read_file(file);
	let name = file.replace(".py", "");

	let pkg = wix::lang::get_info(code, file.to_string(), name);

	println!("{:?}", pkg);
}

// function to read from a file
fn read_file(path: &str) -> String {
	let mut file = std::fs::File::open(path).unwrap();
	let mut contents = String::new();
	std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
	contents
}