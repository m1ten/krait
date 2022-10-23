use crate::{exit, krait};

use mlua::{serde::Serializer, Error, Lua, Table, Value};

pub struct LuaState {
    pub lua: Lua,
}

impl LuaState {
    fn lua_init(self) -> Result<Lua, Error> {
        let lua = self.lua;

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

    pub fn gen_lua(name_t: String, table: Table) -> Vec<String> {
        // get the key value pairs from the table
        // order the pairs by key

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
            } else {
                eprintln!("Error: key is not a string");
                exit!(1);
            };

            match value {
                Value::Table(t) => {
                    // if the value is a table, call gen_lua recursively
                    let key_t = format!("{}.{}", name_t, key);
                    let script = Self::gen_lua(key_t, t);

                    script.into_iter().for_each(|line| {
                        result.push(line);
                    });
                }
                Value::String(s) => {
                    let s = s.to_str().unwrap();

                    // if the value is a string, add the key value pair to the result vector
                    result.push(format!("{}.{} = \"{}\"\n", name_t, key, s));
                }
                Value::Integer(i) => {
                    // if the value is an integer, add the key value pair to the result vector
                    result.push(format!("{}.{} = {}\n", name_t, key, i));
                }
                Value::Number(n) => {
                    // if the value is a number, add the key value pair to the result vector
                    result.push(format!("{}.{} = {}\n", name_t, key, n));
                }
                Value::Boolean(b) => {
                    // if the value is a boolean, add the key value pair to the result vector
                    result.push(format!("{}.{} = {}\n", name_t, key, b));
                }
                Value::Function(_) => {
                    // if the value is a function, add the key value pair to the result vector
                    result.push(format!("function {}.{}()\nend\n", name_t, key));
                }
                Value::Nil => {
                    // if the value is nil, add the key value pair to the result vector
                    result.push(format!("{}.{} = nil\n", name_t, key));
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

            // get the number of tables under krait
            let table_count = table_names.len();
            let mut i = 0; 

            // TODO: add support for nested tables
            for table_name in table_names {
                i = i + 1;

                // replace all instances of krait.table_name with &table_name[0..1]
                for line in &mut result {
                    *line = line.replace(&format!("krait.{}", table_name), &table_name[0..1]);
                }

                if i == table_count {
                    // if this is the last table, add a line to the beginning defining the variable
                    result.insert(0, format!("local {} = krait.{}\n", &table_name[0..1], table_name));
                } else {
                    // if this is not the last table, add a line to the beginning defining the variable and a newline
                    result.insert(0, format!("local {} = krait.{}\n\n", &table_name[0..1], table_name));
                }
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
        }
 
        result
    }
}
