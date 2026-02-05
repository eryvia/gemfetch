mod characters;
mod render;

use characters::all_characters;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let ch = all_characters()
        .choose(&mut rng)
        .expect("no characters configured");

    render::print_character(ch);
}
