// use mlua::{Lua, LuaSerdeExt};

// use crate::{KraitConfig, exit, pkg::Pkg};

// pub fn set_krait_global(lua: Lua, config: Option<KraitConfig>, pkg: Option<Pkg> ) -> Lua {
// 	let globals = lua.globals();

// 	// create a new table for krait
// 	let krait_table = match lua.create_table() {
// 		Ok(t) => t,
// 		Err(e) => {
// 			eprintln!("Error: Could not create krait table: {}", e);
// 			exit!(1);
// 		}
// 	};

// 	let config_value = match lua.to_value(&config) {
// 		Ok(x) => x,
// 		Err(e) => {
// 			eprintln!("Error: Could not convert krait config to lua value: {}", e);
// 			exit!(1);
// 		}
// 	};

// 	// set krait.config table
// 	match krait_table.set("config", config_value) {
// 		Ok(_) => (),
// 		Err(e) => {
// 			eprintln!("Error: Could not set krait.config table: {}", e);
// 			exit!(1);
// 		}
// 	};

// 	let pkg_value = match lua.to_value(&pkg) {
// 		Ok(x) => x,
// 		Err(e) => {
// 			eprintln!("Error: Could not convert krait pkg to lua value: {}", e);
// 			exit!(1);
// 		}
// 	};

// 	// set krait.pkg table
// 	match krait_table.set("pkg", pkg_value) {
// 		Ok(_) => (),
// 		Err(e) => {
// 			eprintln!("Error: Could not set krait.pkg table: {}", e);
// 			exit!(1);
// 		}
// 	};

// 	// set krait table as global
// 	match globals.set("krait", krait_table) {
// 		Ok(_) => (),
// 		Err(e) => {
// 			eprintln!("Error: Could not set krait table as global: {}", e);
// 			exit!(1);
// 		}
// 	};

// 	lua
	
// }