#[path = "lang/py.rs"]
pub mod lang;

#[path = "lib/args.rs"]
pub mod args;

#[derive(Debug)]
pub struct Information {
	pub name: String,
	pub author: String,
	pub version: String,
	pub description: String
}

#[derive(Debug)]
pub struct Package {
	pub name: String,
	pub version: String,
	pub url: String
}