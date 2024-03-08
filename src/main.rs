use story_elements::{Story, Mission, Character};

use crate::{functions::get_random_tag, generate_elements::{generate_char, generate_tags}, globals::create_faction_tag_list};


mod story_elements;
mod globals;
mod functions;
mod generate_elements;

fn main() {
    generate_story();
}



fn generate_story() {
    create_faction_tag_list();
    println!("=======================");
    let mut story=Story {
        mission: None,
        mission_client: None,
        mission_opponant: None,
        mission_antagonist: None
    };

    println!("-- GENERATE MISSION -- ");
    // -- Mission
    let mission_type = "assassination";
    let mission_target = generate_char(3);
    let mission = Mission{
        mission_type: mission_type.to_string(),
        target: mission_target,
        tags : generate_tags(1)
    };
    
    story.mission = Some(mission.clone());
    

    // -- Characters
    println!("- Characters involved -");

    // Client    
    let mut mission_client:Character = generate_char(4);
    story.mission_client = Some(mission_client.clone());
 
     // Adversaire
     let mut mission_opponant = generate_char(3);
     story.mission_opponant = Some(mission_opponant.clone());
     
    // Antagoniste
    let mut mission_antagonist = generate_char(2);
    story.mission_antagonist = Some(mission_antagonist.clone());
    
    // Add Mission Tag to all characters.
    //mission_client.add_tag(mission_tag.to_string());  // OSKOUR //EXPLAIN
    let mission_tag:String = get_random_tag(&mission.tags).clone();
    mission_client.tags.push(mission_tag);      // Cant do it with add_tag because borrowing... oO  // EXPLAIN PLEASE
    let mission_tag:String = get_random_tag(&mission.tags).clone();
    mission_opponant.tags.push(mission_tag); 
    let mission_tag:String = get_random_tag(&mission.tags).clone();
    mission_antagonist.tags.push(mission_tag); 
    
    // Description
    mission_client.description();
    mission_opponant.description();
    mission_antagonist.description();  
    mission.description();   

    // -- Relations.
    println!("- Relationships -");
    /* 
    create_relationship("hostile", mission_client, mission_opponant);
    create_relationship("hierarchical", mission_opponant, mission_antagonist);
    create_relationship("random", mission_opponant, mission_client);
    create_relationship("possible", mission_opponant, mission_target);
    */

    // -- Agents de l'Antagoniste
    println!("- Agents of Antagonist -");
    println!("Get tags from Antagonist");
    println!("Create agents from tags");
    println!("Create agents from faction");

    // -- Location 
    println!("- Location -");
    println!("Get tags from Opponant, Antagonist & target");
    println!("Create location from tags");
    println!("Location is linked to Opponant, Antagonist or Target");

    // -- Zones
    println!("- Zones of Location -");

    // -- Generate Event MissionEnd.
    println!("-- MISSION DEBRIEF --");
}


fn generate_relationship(relation_type:&'static str, character:&'static str, other_character:&'static str) {
    println!("Relation between {} and {} is {}", character, other_character, relation_type)
}



