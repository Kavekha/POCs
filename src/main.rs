fn main() {
    let world = generate_world();
    generate_mission(world);
}

pub fn generate_mission(mut world: World){
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
    println!("--- Generating Mission ---");
    let target = generate_character("Meathead".to_string());
    world.characters.push(target);
    println!("world characters : {:?}.", world.characters);
}

pub fn generate_character(
    name: String
) -> Character {
    let character = Character{
        name
    };
    return character
}

#[derive(Debug, Clone)]
pub struct Character{
    pub name: String
}


pub fn generate_world() -> World {
    let world = World {
        characters: Vec::new()
    };
    return world
}

#[derive(Debug, Clone)]
pub struct World{
    pub characters: Vec<Character>
}
