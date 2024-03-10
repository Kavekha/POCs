fn main() {
    //generate_story();
    generate_simplified_story();
}


fn generate_story() {
    let fixer_name = "Alex Reacher";
    let fixer_race = "elf";

    let client_name = " Sacristan";
    let client_race = "Human";
    let client_faction = "Olaya cartel";
    let client_tags = ["petite", "hardwork", "ruthless", "efficiency"];
    let delivery_point = "Gethsemane cemetery";
    let client_faction_tags = ["stranger", "drug", "tempo", "military", "discretion"];
    let client_boss = "Uribe";
    let client_boss_tags = ["diplomat", "traveler", "small talk", "ruthless"];

    let job_type = "snatch";    // Capture
    let job_objective_major = "bring the product producer";
    let faction_opponant = "Komungo Ring";
    let faction_opponant_flags = ["stranger", "survival", "slavetrade", "drug", "ruthless"];
    let job_price = 6000;
    let factions_involved = ["Laesa", "Yakusa", "Komungo Ring"];

    let second_target_name = "José Vilamour";
    let second_target_race = "elf";
    let second_target_work_place = "Amazonian consulate";
    let second_target_contact = "Olaya cartel bagman";
    let second_target_hideout = "Renton flat";
    let second_target_tags = ["bureaucrat", "bored", "homesick", "possessed"];

    let second_faction_opponant = "Yakuza";
    let second_faction_opponant_tags = ["stranger", "humanis", "retaliation", "gambling", "protection"];

    let target_job = "dealer";
    let target_race = "elf";
    let target_name = "Smalls";
    let target_work_places = "Clubs";
    let target_activities = "Sell Flipside derms."; // Flipside = new drug like Tempo drug.
    let target_flags = ["drug", "club", "flipside", "survival"];
    let target_allies = "Laesa";        // Drug cartel?

    let target_hideout_name = "The Daisy Chain"; // Private club.
    let target_hideout_tags = ["elf", "drug", "music", "tempo", "successful"];
    let target_hideout_faction_owner = "Laesa";


    println!("=== The meeting ===");
    println!("Your Fixer, the {} {}, has contacted you for a new run.", fixer_name, fixer_race);
    println!("The client wants to stay anonymous, and pay a fee for this.");
    println!("This is an investigate and {} job.", job_type);
    println!("The target is a {}, an {} called {}.", target_job, target_race, target_name);
    println!("They work on various {} clubs, where they {}.", target_race, target_activities);
    println!("Your job is to {} to the anonymous Johnson.", job_objective_major);
    println!("The {} seems to be involved, looking for the same target. Be careful.", faction_opponant);
    println!("Job payment is {}. {} in advance.", job_price, job_price / 4);

    println!("=== Event #1 ===");
    println!("[This event is trigger when looking for {} in some {}.]", target_name, target_work_places);
    println!("[Interrogating people with tags linked to those of {} will lead to their hideout: {}.", target_name, target_hideout_name);
    println!("[Taking too much time or heat will lead to an encounter with {:?}, looking for {} as well.", factions_involved, target_name);

    println!("=== Event #2 ===");
    println!("[This event is trigger when {} hideout is known and the Runners go to {}.]", {target_name}, {target_hideout_name});
    println!("[In {}, members of {} are interrogating {} and are about to kill them.]", target_hideout_name, faction_opponant, target_name);
    println!(">> If {} is saved, then he gives the runners information for a trip to the streetdoctor.", target_name);
    println!(">> If {} is dead, they may find infos at his home.", target_name);
    println!("{} buys his flipside from {}.", target_name, second_target_name);
    println!("{} works at {}, and get his stuff from a connexion there, a bagman from {} who's skimming his bosses.", second_target_name, second_target_work_place, client_faction);
    println!("He can be found at {}.", second_target_hideout);

    println!("=== Climax ===");
    println!("[If contact is made with the Johnson, orders to capture {} and bring him to {}. Dead is fine, but at half price.]", second_target_name, delivery_point);
    println!("[{} can be seen at {} exterior, but they aren't hostiles... until their comrads can ambush them inside.]", second_faction_opponant, second_target_hideout);
    println!("[Upstairs, {} are torturing {}, when he suddenly rises as a Possessed Man!!]", second_faction_opponant, second_target_name);
    println!("[The possessed {} is really dangerous and start killing {}, or maybe even the Runners.]", second_target_name, second_faction_opponant);
    println!("[He flees to the lab section of {}, where the chemical smell make anyone poisonned if not protected.]", second_target_hideout);
    println!("[The Spirit that possessed {} try to kill himself by blowing the lab, but if they can't do it they will leave his body and let him slumber from the experience.]", second_target_name);
    println!("[If the lab doesn't explosed, raw tempo can be found for a 60K value if sold through a specific contact.]");

    println!("=== Epilogue ===");
    println!("[Back to the Fixer {}, the run is debriefed.]", fixer_name);
    println!(">> If {} dead with a proof of it but no corpse, only half the remaining money is paid.", second_target_name);
    println!(">> If they have him captured or dead, they will be asked to come to {} the delivery point.", delivery_point);
    println!("[At {}, a small group with body guards await them, near a freshly dig grave. A woman comes to them : she's their anonymous client, {}.]", delivery_point, client_name);
    println!("They'll have a bonus on their paid for their service.");
    println!("The rest of the group talk to {}, but can't get any information from him as he was under influence when dealing with flipside.", second_target_name);
    println!("The big boss of the group, {}, ask the runners to execute {}. If they don't do it, he will do it himself.", client_boss, second_target_name);
    println!("The big boss and his allies walk awak, {} pay the runners and be sure to check them again.", client_name);
}

fn generate_simplified_story(){
    println!("=== BRIEFING ===");
    println!("Generate : Fixer.");
    println!("Mission:");
    println!("Anonymous client.");
    println!("objectif: identify and found target and snatch him to a delivery point.");
    println!("hint: a base target who would know who is the target and where he is.");
    println!("complication: a criminal faction is in the mix.");

    let fixer = generate_character("Alex Reacher".to_string(), Race::Elf, IsKnown::Known);
    let client = generate_character("Sacristan".to_string(), Race::Human, IsKnown::Unknown);
    let mut mission = generate_mission(client, fixer, IsKnown::Unknown);
    //First Segment
    let first_target = generate_character("José Vilamour".to_string(), Race::Elf, IsKnown::Unknown);
    let first_faction_leader = generate_character("Komungo Leader".to_string(), Race::Dwarf, IsKnown::Unknown);
    let first_faction = generate_faction("KomungoRing".to_string(), MasterFaction::KomungoRing, first_faction_leader);
    let first_hint = generate_character("Smalls".to_string(), Race::Elf, IsKnown::Known);
    let first_segment = generate_segment(MissionType::Identify, first_target, first_faction, first_hint);
    mission.segments.push(first_segment);
    //Second segment.
    /* 
    let second_faction_leader = generate_character("Second Komungo Leader".to_string(), Race::Human, IsKnown::Unknown);
    let second_faction = generate_faction("KomungoRing".to_string(), MasterFaction::KomungoRing, second_faction_leader);
    let second_segment = generate_segment(MissionType::Find, first_target, second_faction, first_hint);
    mission.segments.push(second_segment);
    */

    println!("======================================");
    run_mission(mission);

}

fn run_mission(
    mission: Mission
) {
    println!("== Briefing is starting ==");
    println!("You meet with your fixer, {}, for a new run.", mission.fixer.name);
    // Known-Unkwnown.
    println!("According to him, the client is anonymous, but payement will be made to compensate for their privacy.");
    for segment in mission.segments {
        match segment.mission_type {
            MissionType::Identify => {
                println!("You have to identified someone.");   //Reason?
                println!("To track them, you may have to ask {}.", segment.hint.name);
                println!("Careful, the {} seems to look for it too!", segment.opposing_faction.name);
            },
            _ => println!("That's all.")
        };
    };
}

enum Race {
    Human,
    Elf,
    Dwarf
}
pub struct Character {
    pub name: String,
    pub race: Race,
    pub known: IsKnown
}

fn generate_character(
    name: String, 
    race: Race,
    known: IsKnown
) -> Character {
    let character = Character{
        name, 
        race,
        known
    };
    return character
}

enum IsKnown {
    Known,
    Unknown
}

fn generate_mission(
    client: Character,
    fixer: Character,
    known_by_fixer: IsKnown,
) -> Mission {
    let mission = Mission{
        client,
        fixer,
        known_by_fixer,
        segments : Vec::new()
    };
    return mission
}

pub struct Mission {
    pub client: Character,
    pub fixer: Character,
    pub known_by_fixer: IsKnown,
    pub segments: Vec<Segment>
}

enum MissionType {
    Identify,
    Find,
    KillOrCapture,
    Delivery
}

fn generate_segment (
    mission_type: MissionType,
    target: Character,
    opposing_faction: Faction,
    hint: Character
) -> Segment {
    let segment = Segment{
        mission_type,
        target,
        opposing_faction,
        hint
    };
    return segment
}

pub struct Segment {
    pub mission_type: MissionType,
    pub target: Character,
    pub opposing_faction: Faction,
    pub hint: Character
}

enum MasterFaction{
    Yakuza,
    LaesaGang,
    OlayaCartel,
    KomungoRing
}

fn generate_faction(
    name: String, 
    master_faction: MasterFaction,
    leader: Character
) -> Faction {
    let faction = Faction{
        name, 
        master_faction,
        leader
    };
    return faction
}

pub struct Faction {
    pub name: String,
    pub master_faction: MasterFaction,
    pub leader: Character
}