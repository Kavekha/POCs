use rand::Rng;

use crate::{globals::{FACTION_ELEMENTS, FACTION_LIST, NAME_ELEMENTS, NAME_LIST, TAG_ELEMENTS, TAG_LIST}, story_elements::Character};

pub fn generate_char(nb_tags:i32) -> Character {
    let character = Character{
        name: generate_name(),
        faction: generate_faction(),
        tags: generate_tags(nb_tags) 
    };
    return character
}

pub fn generate_name() -> String {
    let mut rng = rand::thread_rng();
    let rand= rng.gen_range(0..NAME_ELEMENTS);
    let name = &NAME_LIST[rand];
    return format!("{}", name)
}

pub fn generate_faction() -> String {
    let mut rng = rand::thread_rng();
    let rand= rng.gen_range(0..FACTION_ELEMENTS);
    let faction = &FACTION_LIST[rand];
    return format!("{}", faction)
}

pub fn generate_tags(nb:i32) -> Vec<String> {
    let mut tags = Vec::new();
    for i in 0..nb {
        let mut rng = rand::thread_rng();
        let rand= rng.gen_range(0..TAG_ELEMENTS);
        let tag = &TAG_LIST[rand];
        tags.push(tag.to_string())
    }
    return tags
}