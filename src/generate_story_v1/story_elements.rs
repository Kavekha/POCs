#[derive(Debug, Clone)]
pub struct Character{
    pub name: String,
    pub faction: String,
    pub tags: Vec<String>
}
impl Character {
    pub fn description(&self) {
        println!("NPC {}: My faction is {}. Tags are {:?}", &self.name, &self.faction, &self.tags);
    }
    pub fn add_tag(mut self, tag:String) {
        self.tags.push(tag);
    }
}


#[derive(Debug, Clone)]
pub struct Mission {
    pub mission_type:String,
    pub target: Character,
    pub tags: Vec<String>
}
impl Mission {
    pub fn description(&self){
        println!("This is an {} mission. Your target is {}, a member of {}.", &self.mission_type, &self.target.name, &self.target.faction);
        println!("[Mission themes]: {:?}", &self.tags);
        println!("[Target themes]: {:?}", &self.target.tags);
    }
}


pub struct Story{
    pub mission: Option<Mission>,
    pub mission_client: Option<Character>,
    pub mission_opponant: Option<Character>,
    pub mission_antagonist: Option<Character>
}
