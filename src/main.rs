#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unreachable_code)]

fn main() {
    let info = wix::structs::Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "wix - cross platform package manager".to_string(),
        license: "zlib".to_string(),
        repository: "github.com/m1ten/wix".to_string()
    };

    let args = wix::args::Arguments::new(info.clone());

    // code for info
    //let info_code = wix::lang::struct_to_code("info".to_string(), "no".to_string(), vec!["hello".to_string()]);

    //println!("{}", info_code);

    let struct_name = "Information".to_string();
    let struct_contents = wix::structs::Information::get_field_type(Some(info));

    let struct_code = wix::lang::struct_to_code(struct_name, struct_contents);

    println!("{}", struct_code);
}    
