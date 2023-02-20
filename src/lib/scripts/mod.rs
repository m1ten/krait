use crate::{exit, structs::KraitMain};

use mlua::{DeserializeOptions, Error, Lua, LuaSerdeExt, Table, Value};

/// Generic name for future scripting languages support
pub trait KraitScript {
    /// Lua is default but can be changed to other scripting languages
    fn init(lua: Option<Lua>) -> Result<Lua, Error> {
        let lua = match lua {
            Some(l) => l,
            None => Lua::new(),
        };

        // create a new krait table
        let krait_table = lua.create_table()?;

        // create a new config table
        let config_table = lua.create_table()?;

        // set the config table as a field of the krait table
        krait_table.set("config", config_table)?;

        // create a new pkg table
        let pkg_table = lua.create_table()?;

        // set the pkg table as a field of the krait table
        krait_table.set("pkg", pkg_table)?;

        // create a new manifest table
        let manifest_table = lua.create_table()?;

        // set the manifest table as a field of the krait table
        krait_table.set("manifest", manifest_table)?;

        // set the krait table as a global
        lua.globals().set("krait", krait_table)?;

        Ok(lua)
    }

    fn k_fmt(&self) -> Vec<String> {
        todo!();
    }

    fn parse(script: &String) -> Result<KraitMain, Error> {
        let lua = KraitMain::init(None)?;

        lua.load(script).exec()?;

        let globals = lua.globals();
        let krait_t = globals.get::<_, mlua::Table>("krait")?;

        let options = DeserializeOptions::new()
            .deny_unsupported_types(false)
            .deny_recursive_tables(false);

        let krait_struct: KraitMain = match lua.from_value_with(Value::Table(krait_t), options) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("Big Boy Error: {}", e);
                exit!(1);
            }
        };

        Ok(krait_struct)
    }
}

pub struct LuaState;

impl LuaState {
    #[deprecated = "inefficient method of generating lua code"]
    pub fn gen_lua(name_t: String, table: Table) -> Vec<String> {
        let pairs: mlua::TablePairs<Value, Value> = table.clone().pairs();

        let mut result: Vec<String> = Vec::new();

        // iterate over the key value pairs
        for pair in pairs {
            // get the key and value
            let (key, value) = match pair {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    exit!(1);
                }
            };

            let key: String = if let Value::String(s) = key {
                s.to_string_lossy().to_string()
            } else if let Value::Integer(i) = key {
                i.to_string()
            } else {
                eprintln!("Error: invalid key type");
                exit!(1);
            };

            match value {
                Value::Table(t) => {
                    // if the value is a table, call gen_lua recursively
                    // let key_t = format!("{}.{}", name_t, key);

                    let key_t = format!("{}[\"{}\"]", name_t, key);

                    // if the keys under the table are strings, then we can use the dot syntax
                    // otherwise we have to use the bracket syntax

                    let pairs: mlua::TablePairs<Value, Value> = t.clone().pairs();

                    let mut array: String = key_t.clone() + " = {";

                    for pair in pairs {
                        let (k, v) = match pair {
                            Ok(p) => p,
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                exit!(1);
                            }
                        };

                        // check if the key is a integer
                        match k {
                            Value::Integer(_) | Value::Number(_) => match v {
                                Value::String(s) => {
                                    array += &format!("\"{}\",", s.to_string_lossy());
                                }
                                Value::Integer(i) => {
                                    array += &format!("{},", i);
                                }
                                Value::Number(n) => {
                                    array += &format!("{},", n);
                                }
                                Value::Boolean(b) => {
                                    array += &format!("{},", b);
                                }
                                Value::Table(_) => {
                                    eprintln!("Error: invalid value type");
                                    exit!(1);
                                }
                                _ => {
                                    eprintln!("Error: invalid value type");
                                    exit!(1);
                                }
                            },
                            Value::String(_) => {
                                continue;
                            }
                            _ => {
                                eprintln!("Error: invalid key type");
                                exit!(1);
                            }
                        }
                    }

                    let mut r_table = Self::gen_lua(key_t.clone(), t);
                    array = array.trim_end_matches(',').to_string();
                    array += "}\n";

                    // remove the table from the r_table which is in the format of "a.1 = x" and "a.2 = y"

                    r_table.clone().into_iter().for_each(|r| {
                        // check if between . and = there is a number
                        let mut split = r.split('=');

                        let variable = split.next().unwrap().trim_end();

                        // check if the end is a number and if it is, remove the line
                        if variable.split('.').last().unwrap().parse::<i32>().is_ok() {
                            r_table.remove(r_table.iter().position(|x| x == &r).unwrap());
                        }
                    });

                    // check if there is nothing between the brackets
                    if array != format!("{} = {{}}\n", key_t) {
                        // add the array to r_table
                        r_table.push(array);
                    }

                    result.append(&mut r_table);
                }
                Value::String(s) => {
                    let s = s.to_str().unwrap();

                    // if the value is a string, add the key value pair to the result vector
                    // result.push(format!("{}.{} = \"{}\"\n", name_t, key, s));

                    result.push(format!("{}[\"{}\"] = \"{}\"\n", name_t, key, s));
                }
                Value::Integer(i) => {
                    // if the value is an integer, add the key value pair to the result vector
                    // result.push(format!("{}.{} = {}\n", name_t, key, i));

                    result.push(format!("{}[\"{}\"] = {}\n", name_t, key, i));
                }
                Value::Number(n) => {
                    // if the value is a number, add the key value pair to the result vector
                    // result.push(format!("{}.{} = {}\n", name_t, key, n));

                    result.push(format!("{}[\"{}\"] = {}\n", name_t, key, n));
                }
                Value::Boolean(b) => {
                    // if the value is a boolean, add the key value pair to the result vector
                    // result.push(format!("{}.{} = {}\n", name_t, key, b));

                    result.push(format!("{}[\"{}\"] = {}\n", name_t, key, b));
                }
                Value::Function(_) => {
                    // if the value is a function, add the key value pair to the result vector
                    // result.push(format!("function {}.{}()\nend\n", name_t, key));

                    result.push(format!("{}[\"{}\"] function ()\nend\n", name_t, key));
                }
                Value::Nil => {
                    // if the value is nil, add the key value pair to the result vector
                    // result.push(format!("{}.{} = nil\n", name_t, key));

                    result.push(format!("{}[\"{}\"] = nil\n", name_t, key));
                }
                _ => {
                    let type_ = value.type_name();
                    // if the value is not a table, string, integer, or boolean, print an error
                    eprintln!("Error: unsupported type: {}", type_);
                    exit!(1);
                }
            }
        }

        // check if top level table is krait
        if name_t == "krait" {
            // for every sub table, add a line to the beginning defining a variable named the first letter of the table
            // and set it equal to the table (e.g. c = krait.config)

            // get the table names under krait
            let table_names: Vec<String> = table
                .clone()
                .pairs::<Value, Value>()
                .map(|pair| {
                    let (key, _) = pair.unwrap();
                    let key: String = if let Value::String(s) = key {
                        s.to_string_lossy().to_string()
                    } else {
                        eprintln!("Error: key is not a string");
                        exit!(1);
                    };
                    key
                })
                .collect();

            let mut shorts: Vec<String> = Vec::new();

            // TODO: add support for nested tables
            for table_name in table_names {
                // check if the short name is already taken
                let short: String;

                if shorts.contains(&table_name[0..1].to_string()) {
                    // if the short name is taken, add a number to the end of the short name
                    let mut i = 1;
                    loop {
                        let short_name = table_name[0..i].to_string();
                        if !shorts.contains(&short_name) {
                            short = short_name;
                            break;
                        }
                        i += 1;
                    }
                } else {
                    // if the short name is not taken, use it
                    short = table_name[0..1].to_string();
                }

                shorts.push(short.clone());

                // replace all instances of krait.table_name with &table_name[0..1]
                for line in &mut result {
                    // *line = line.replace(&format!("krait.{}", table_name), &short);

                    *line = line.replace(&format!("krait[\"{}\"]", table_name), &short);
                }

                // if this is not the last table, add a line to the beginning defining the variable
                // result.insert(0, format!("local {} = krait.{}\n", short, table_name));

                result.insert(0, format!("local {} = krait[\"{}\"]\n", short, table_name));
            }

            // move functions to the end of the file
            let mut functions: Vec<String> = Vec::new();
            let mut i = 0;
            while i < result.len() {
                if result[i].contains("function") {
                    functions.push(result.remove(i) + "\n");
                } else {
                    i += 1;
                }
            }

            result.push("\n".to_string());
            result.append(&mut functions);

            // add newline after local variables
            result.insert(shorts.len(), "\n".to_string());
        }

        result
    }
}


// TODO: rewrite the whole impl, use serde parser