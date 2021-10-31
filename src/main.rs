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
}