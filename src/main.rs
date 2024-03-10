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
    let target_two = world.create_character();
    mission.add_target(target_two);
    let target_three = world.create_character();
    mission.add_target(target_three);
    //Type
    mission.set_objective(MissionType::Assassination);
    //Payment
    mission.set_payment(3000);

    //Read debriefing.
    mission.briefing();

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
    base_payment: i32
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
    pub fn briefing(self){
        match self.fixer {
            None => {
                println!("You heard about someone looking for runners, but can't quite identified them.");
                return 
            },
            _ => println!("Fixer says:")
        }
        match self.client {
            None => {
                println!("Our client wants to remain anonym so we will call him 'Mr Johnson' as usual.");
                println!("I'll be sure to apply the anonymous fee on him.");
            },
            _ => println!("Our client is SomeOne.")
        } 
        match self.target {
            None => {
                println!("Our client has been enable to identify their target, and will come back to us when they do."); // vec.last()
                return 
            },
            _ => {
                println!("The target is Meathead."); //println!("The target is Meathead, an ancient member of the Hell Angels gang going solo on the drug trade.");
            }            
        }        
        println!("Kill him and come back in on piece to get paid.");
        println!("Payment is {} nuyens.", self.base_payment); //println!("Budget is 3000 nuyens, 1000 in advance, plus a 400 nuyens fee for not being polite enough");
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
