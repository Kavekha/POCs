fn main() {
    let mut world = World::new();
    generate_story(&mut world);
}


pub fn generate_story(world: &mut World){
    println!("--- Fake-generated Mission ---");

    println!("Fixer says:");
    println!("Our client wants to remain anonym so we will call him 'Mr Johnson' as usual.");
    println!("I'll be sure to apply the anonymous fee on him.");
    println!("The target is Meathead, an ancient member of the Hell Angels gang going solo on the drug trade.");
    println!("From the infos given by our mr Johnson, Meathead's hideout is in Anoubarak Station, abandonned a long time ago.");
    println!("He's there with some friends and maybe a few locals he recruited to start his business.");
    println!("Kill him and come back in on piece to get paid.");
    println!("Budget is 3000 nuyens, 1000 in advance, plus a 400 nuyens fee for not being polite enough");

    println!("");
    println!("===========================");
    println!("");

    let mut mission = generate_mission();

    let mut target = world.create_character();
    //println!("target is {:?}", *target);
    mission.add_target(target);
    //mission.add_target(world.create_character());
    let mut other_target = world.create_character();
    mission.add_target(other_target);

}

pub fn generate_mission(
) -> Mission {
    let mut mission = Mission::new();
    return mission
}


pub struct Mission{
    targets: Vec<Character>,
    client: Option<Character>
}
impl Mission{
    pub fn new() -> Mission {
        let mission = Mission {
            targets: Vec::new(),
            client: None
        };
        return mission 
    }
    pub fn add_target(&mut self, target: Character) {
        self.targets.push(target.clone());
    }
    pub fn briefing(self){
        println!("This is a briefing from this mission");
    }
}

 
#[derive(Clone, Debug,)]
pub struct World{
    last_char_id: i32,
    pub characters: Vec<Character>
}
impl World{
    pub fn new() -> World {
        let world = World {
            last_char_id: 0,
            characters: Vec::new()
        };
        return world
    }
    pub fn create_character(&mut self) -> Character {
        let character = Character {};
        self.last_char_id += 1;
        self.characters.push(character.clone());
        //self.characters.push(character.clone());
        return character.clone()
    }
}



#[derive(Clone, Debug)]
pub struct Character {}
