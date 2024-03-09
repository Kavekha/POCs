use std::collections::HashMap;

pub const NAME_ELEMENTS: usize = 4;
pub const NAME_LIST: &'static [&'static str] = &["Firefox", "Chrome", "Brave", "Safari"];

pub const FACTION_ELEMENTS: usize = 4;
pub const FACTION_LIST: &'static [&'static str] = &["Cadavers of the moon", "Dark Necromancers", "Smugglers Compagny", "Corpse Eaters"];

pub const TAG_ELEMENTS: usize = 4;
pub const TAG_LIST: &'static [&'static str] = &["Collector", "Weapon", "Metahumain", "Gang"];

// TAGS BY FACTION
pub fn create_faction_tag_list() -> HashMap<String, [String; 9]> {
    let cadaver_tags = [
        "Survival".to_string(), "Underground".to_string(), "Metahumain".to_string(), "Gang".to_string(), 
        "Food".to_string(), "Protection".to_string(), "Community".to_string(), "Weapon".to_string(), "Canibal".to_string()
    ];
    let necromancer_tags = [
        "Hunter".to_string(), "Underground".to_string(), "Metahumain".to_string(), "Gang".to_string(), "Feral".to_string(), 
        "Canibal".to_string(), "Ritual".to_string(), "Blood".to_string(), "Flesh".to_string()
    ];

    let mut faction_tag_list = HashMap::new();
    faction_tag_list.insert("Cadavers of the moon".to_owned(), cadaver_tags);
    faction_tag_list.insert("Dark Necromancers".to_owned(), necromancer_tags);
    return faction_tag_list
}