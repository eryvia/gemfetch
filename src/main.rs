mod characters;
mod render;

use characters::all_characters;
use rand::seq::SliceRandom;

fn main() {
    let list = get_all_character_quotes();
    
    println!("Total characters loaded: {}", list.len());
    
    // Now you can access individual fields
    if let Some(first) = list.first() {
        println!("The first key is: {}", first.key);
    }

    let mut rng = rand::thread_rng();
    let ch = all_characters()
        .choose(&mut rng)
        .expect("no characters configured");

    render::print_character(ch);
}
