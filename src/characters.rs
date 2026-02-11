use serde::Deserialize;
use std::fs;
use crate

pub struct Config {
    character: Vec<Character>
}

pub struct Character {
    key: String,
    character: String,
    nickname: String,
    quote: String,
    source: String,
    image_path: String,
}

pub fn get_all_character_quotes() -> Vec<Character> {

    let filename = "characters/animanga.toml"; 
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let decoded: Config = toml::from_str(&contents)
        .expect("Error parsing TOML");

    for c in decoded.character {
        println!("{} from {} says: \"{}\"", c.character, c.source, c.quote);
    }

    decoded.character
}

