use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug, Clone)]
pub struct Endpoint {
    pub host: String,
    pub path: String,
    pub method: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub address: String,
    pub endpoints: HashMap<String, Endpoint>,
}

pub fn read_config(file: String) -> Config {
    let file_content = fs::read_to_string(file).expect("Failed to read file");
    serde_json::from_str(&file_content).expect("Failed to parse JSON")
}
