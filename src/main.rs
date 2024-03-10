use rand::prelude::*;

//v1. Generer une mission avec du contenu aleatoire, avec une forme de cohérence. Se traduit en briefing + elements gameplay exploitables.
//v2. Generer plusieurs missions, avec du contenu créé ou réutilisé, qui conserve sa coherence, et avec une progression dans l'apparence de difficulté.
//v3. Forcer la generation des missions autour d'une thématique "compagne", c'est à dire une continuité dans les personnages et factions impliquées.
//v4. Diviser les Missions en scenario, avec des segments / objectifs intermediaires pour permettre une progression de l'intrigue dans le cadre de cette mission.


// 
fn main() {
    let mut world = World::new();
    generate_story(&mut world);
}


pub fn generate_story(world: &mut World){
    println!("--- Fake-generated Mission ---");

    println!("Fixer says:");    // OK
    println!("Our client wants to remain anonym so we will call him 'Mr Johnson' as usual.");   // OK // TODO : pronouns
    println!("I'll be sure to apply the anonymous fee on him.");    // OK // TODO : Pronouns, payment modifier
    println!("The target is Meathead, an ancient member of the Hell Angels gang going solo on the drug trade.");    // OK 
    println!("From the infos given by our mr Johnson, Meathead's hideout is in Anoubarak Station, abandonned a long time ago.");    //OK
    println!("He's there with some friends and maybe a few locals he recruited to start his business.");    // NOK : //TODO : Hint on target army / antagonists.
    println!("Kill him and come back in on piece to get paid.");
    println!("Budget is 3000 nuyens, 1000 in advance, plus a 400 nuyens fee for not being polite enough");
    // TODO : Random info.
    // TODO : Re-use created chars.
    // TODO : Some kind of tags.

    println!("");
    println!("===========================");
    println!("");

    let mut mission = Mission::new();
    //fixer
    let fixer = world.create_character();
    mission.add_fixer(fixer);
    //client
    let client = world.create_character();
    mission.add_client(client);
    //target
    let target = world.create_character();
    mission.add_target(target);
    //Type
    mission.set_objective(MissionType::Assassination);
    //Payment
    mission.set_payment(3000);
    //Location
    let location = world.generate_location();
    mission.add_location(location);
    //Read debriefing.
    mission.briefing();

}

pub fn generate_random_character_name() -> String {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..10);
    let mut name = "".to_string();
    name += match rand {
        0 => "Meathead",
        1 => "Dandy",
        3 => "Nacho",
        4 => "Witch Hand",
        5 => "Omen",
        6 => "Limb Johnny",
        7 => "Absolute Zero",
        8 => "Alice",
        9 => "Ace",
        10 => "Adrian",
        _ => "Akari"
    };
    return name
}

pub fn generate_mission(
) -> Mission {
    let mission = Mission::new();
    return mission
}

pub enum MissionType{
    Assassination
}
pub struct Mission{
    //targets: Vec<Character>,
    target: Option<Character>,
    client: Option<Character>,
    fixer: Option<Character>,
    mission_type: Option<MissionType>,
    base_payment: i32,
    location: Option<Location>
}
impl Mission{
    pub fn new() -> Mission {
        let mission = Mission {
            //targets: Vec::new(),
            target: None, 
            client: None,
            fixer: None,
            mission_type: None,
            base_payment: 0,
            location: None
        };
        return mission 
    }
    pub fn add_target(&mut self, target: Character) {
        //self.targets.push(target.clone());
        self.target = Some(target);
    }
    pub fn add_client(&mut self, client: Character) {
        self.client = Some(client);
    }
    pub fn add_fixer(&mut self, fixer: Character) {
        self.fixer = Some(fixer);
    }
    pub fn set_objective(&mut self, objective: MissionType){
        self.mission_type = Some(objective);
    }
    pub fn set_payment(&mut self, payment: i32){
        self.base_payment = payment;
    }
    pub fn add_location(&mut self, location: Location){
        self.location = Some(location);
    }
    pub fn briefing(self){
        if let Some(fixer) = self.fixer {
            println!("Your fixer {} says:", fixer.name);
        } else {
            println!("You heard about someone looking for runners, but can't quite identified them.");
            return 
        };
        if let Some(client) = self.client {
            println!("Our client is {}.", client.name);
        } else {
            println!("Our client wants to remain anonym so we will call him 'Mr Johnson' as usual.");
            println!("I'll be sure to apply the anonymous fee on him.");
        }
        if let Some(target) = self.target {
            println!("The target is {}.", target.name); 
            println!("{} was a {} for the {}, but is now working for {} as a {}.", target.name, target.previous_background.rank, target.previous_background.faction, target.current_background.faction, target.current_background.rank);
        } else {
            println!("Our client has been enable to identify their target, and will come back to us when they do."); // vec.last()
                return 
        };
        if let Some(mission_type) = self.mission_type {
            match mission_type {
                MissionType::Assassination => println!("Kill him and come back in on piece to get paid."),
                _ => println!("The client has no idea what he wants, so I'll have to come back to you.")
            }
        };
        if let Some(location) = self.location {
            println!("From the infos given by our client, the hideout is in {}, an {} {}.", location.name, location.location_status, location.location_type);
        } else {
            println!("Our client has no idea where anything happens, we'll have to wait for him.");
        };
        println!("Payment is {} nuyens.", self.base_payment); //println!("Budget is 3000 nuyens, 1000 in advance, plus a 400 nuyens fee for not being polite enough");
    }
}


#[derive(Clone, Debug,)]
pub struct World{
    last_char_id: i32,
    pub characters: Vec<Character>,
    character_used_names: Vec<String>,
    faction_used_names: Vec<String>,
    pub locations: Vec<Location>
}
impl World{
    pub fn new() -> World {
        let world = World {
            last_char_id: 0,
            characters: Vec::new(),
            character_used_names: Vec::new(),
            faction_used_names: Vec::new(),
            locations: Vec::new()
        };
        return world
    }
    pub fn create_character(&mut self) -> Character {
        //Name
        let mut name = generate_random_character_name();
        //work bof
        while self.character_used_names.contains(&name) {
            name = generate_random_character_name();
        };
        self.character_used_names.push(name.clone());

        //previous faction
        let mut previous_faction_name = generate_random_faction_name();
        while self.faction_used_names.contains(&previous_faction_name) {
            previous_faction_name = generate_random_faction_name();
        };
        self.faction_used_names.push(previous_faction_name.clone());

        //current faction
        let mut current_faction_name = generate_random_faction_name();
        while self.faction_used_names.contains(&current_faction_name) {
            current_faction_name = generate_random_faction_name();
        };
        self.faction_used_names.push(current_faction_name.clone());

        let character = Character {
            name: name,
            previous_background: generate_random_background(previous_faction_name.clone()),
            current_background: generate_random_background(current_faction_name.clone())
        };
        self.last_char_id += 1;
        self.characters.push(character.clone());
        //self.characters.push(character.clone());
        return character.clone()
    }
    pub fn generate_location(&mut self) -> Location {
        let location = Location {
            name: "Anoubarak Station".to_string(),
            location_type: "Forgotten station".to_string(),
            location_status: "Abandonned".to_string()
        };
        self.locations.push(location.clone());
        return location.clone()
    }
}

pub fn generate_random_background(faction_name: String) -> Background {
    let background = Background{
        faction: faction_name,
        rank: generate_random_rank_name()
    };
    return background
}

pub fn generate_random_faction_name() -> String {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..10);
    let mut name = "".to_string();
    name += match rand {
        0 => "Desolation Angels",
        1 => "405 Hellhounds",
        3 => "Black Rains",
        4 => "Crimson Crush",
        5 => "Octogone",
        6 => "Société du Lotus Blanc",
        7 => "Garde du Serpent Vert",
        8 => "Halloweeners",
        9 => "Razor Heads",
        10 => "Scatterbrains",
        _ => "Troll Killers"
    };
    return name
}

pub fn generate_random_rank_name() -> String {
    let mut rng = rand::thread_rng();
    let rand = rng.gen_range(0..10);
    let mut name = "".to_string();
    name += match rand {
        0 => "Leader",
        1|2|3 => "lieutenant",
        4|5|6 => "Enforcer",
        _ => "meatbag"
    };
    return name
}

#[derive(Clone, Debug)]
pub struct Location{
    pub name: String,
    pub location_type: String,
    pub location_status: String
}

#[derive(Clone, Debug)]
pub struct Background {
    pub faction: String,
    pub rank: String
}

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    previous_background: Background,
    current_background: Background
}
