use rand::prelude::*;

pub fn get_random_tag(tags:&Vec<String>) -> String {
    let nb_tags = tags.len();
    println!("This list has {} tags", nb_tags);
    let mut rng = rand::thread_rng();
    let rand= rng.gen_range(0..nb_tags);
    let tag = tags[rand].clone();
    return tag
}