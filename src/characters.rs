use serde::Deserialize;
use std::fs;

#[derive(Deserialize)] 
pub struct Config {
    pub character: Vec<Character> 
}

#[derive(Deserialize, Clone)]
pub struct Character {
    pub key: String,
    pub character: String,
    pub nickname: String,
    pub quote: String,
    pub source: String,
    pub image_path: String,
}

pub fn get_all_character_quotes() -> Vec<Character> {
    let filename = "characters/animanga.toml"; 
    
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let decoded: Config = toml::from_str(&contents)
        .expect("Error parsing TOML");

    decoded.character
}