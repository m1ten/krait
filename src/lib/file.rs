use std::{fs::File, io::{self, Read, Write}};

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