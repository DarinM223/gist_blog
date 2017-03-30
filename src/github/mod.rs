use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct File {
    pub size: i32,
    pub raw_url: String,
    pub language: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub login: String,
    pub id: i32,
}

#[derive(Deserialize, Debug)]
pub struct Gist {
    pub url: String,
    pub id: String,
    pub description: String,
    pub owner: Owner,
    pub files: HashMap<String, File>,
}
