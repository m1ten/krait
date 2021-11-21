use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct Information {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub repository: String,
}

impl Information {
    pub fn get_field_type(info: Option<Information>) -> IndexMap<String, String> {
        let info = match info {
            Some(i) => i,
            None => {
                let mut map = IndexMap::new();
                map.insert("name".to_string(), "String".to_string());
                map.insert("author".to_string(), "String".to_string());
                map.insert("version".to_string(), "String".to_string());
                map.insert("description".to_string(), "String".to_string());
                map.insert("license".to_string(), "String".to_string());
                map.insert("repository".to_string(), "String".to_string());
                return map
            }
        };
        let mut map = IndexMap::new();
        map.insert("name".to_string(), info.name.clone());
        map.insert("author".to_string(), info.author.clone());
        map.insert("version".to_string(), info.version.clone());
        map.insert("description".to_string(), info.description.clone());
        map.insert("license".to_string(), info.license.clone());
        map.insert("repository".to_string(), info.repository.clone());
        map
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub url: String,
    pub _type: String,
    pub verified: bool,
    pub dependency: String,
    pub dependencies: Vec<String>,
}