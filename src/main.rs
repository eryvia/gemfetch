mod characters;
mod render; 

use characters::get_all_character_quotes;
use rand::seq::IndexedRandom;

fn main() {
    let mut rng = rand::rng();
    let ch_list = get_all_character_quotes();

    if ch_list.is_empty() {
        println!("No characters found!");
        return;
    }

    if let Some(random_ch) = ch_list.choose(&mut rng) {
        render::print_character(random_ch);
    }

    
}