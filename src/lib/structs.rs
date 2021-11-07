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
    pub fn get_field_type(info: Option<Information>) -> Vec<[String; 2]> {
        let info = match info {
            Some(i) => i,
            None => {
                return vec![
                    ["name".to_string(), "String".to_string()],
                    ["author".to_string(), "String".to_string()],
                    ["version".to_string(), "String".to_string()],
                    ["description".to_string(), "String".to_string()],
                    ["license".to_string(), "String".to_string()],
                    ["repository".to_string(), "String".to_string()],
                ]
            }
        };
        return vec![
            ["name".to_string(), info.name.clone()],
            ["author".to_string(), info.author.clone()],
            ["version".to_string(), info.version.clone()],
            ["description".to_string(), info.description.clone()],
            ["license".to_string(), info.license.clone()],
            ["repository".to_string(), info.repository.clone()],
        ];
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub url: String,
    pub verified: bool,
    pub dependencies: Vec<String>,
}
