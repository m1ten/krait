#[path = "lang/py.rs"]
pub mod lang;

#[path = "lib/args.rs"]
pub mod args;

#[path = "lib/structs.rs"]
pub mod structs;

// function to read from a file
pub fn read_file(path: &str) -> String {
	let mut file = std::fs::File::open(path).unwrap();
	let mut contents = String::new();
	std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
	contents
}