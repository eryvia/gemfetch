pub struct Character {
    pub key: &'static str,
    pub name: &'static str,
    pub quote: &'static str,
    pub image_path: &'static str, // local path for now
}

pub fn all_characters() -> &'static [Character] {
    &[
        Character {
            key: "phos",
            name: "Phosphophyllite",
            quote: "Even if it changes me… I still want to be useful.",
            image_path: "assets/phos.jfif",
        },
        Character {
            key: "cinnabar",
            name: "Cinnabar",
            quote: "Don’t come near me.",
            image_path: "assets/HnK.jpg",
        },
    ]
}
