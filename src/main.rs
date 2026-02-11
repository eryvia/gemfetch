mod characters;
mod render;

use characters::get_all_character_quotes;

fn main() {
    let mut rng = rand::thread_rng();
    let ch = get_all_character_quotes()
        .expect("no characters configured");

    //render::print_character(ch);
}
