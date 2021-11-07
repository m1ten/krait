fn main() {
    let info = wix::structs::Information {
        name: "wix".to_string(),
        author: "miten".to_string(),
        version: "0.1.0".to_string(),
        description: "wix - cross platform package manager".to_string(),
        license: "zlib".to_string(),
        repository: "https://github.com/m1ten/wix".to_string()
    };

    let args = wix::args::Arguments::new(info.clone());

    let info_contents = wix::structs::Information::get_field_type(Some(info));

    let mut info_code = wix::lang::struct_to_code("Information".to_string(), info_contents);
    info_code = info_code.replace("Information = {}", "").replace("Information.", "");

    wix::file::writefs(
        "wix.py".to_string(),
        info_code.trim_start().to_string().trim_end().to_string()
    ).unwrap();

}    
