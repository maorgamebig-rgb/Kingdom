use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{self, Write, Read};


use crate::buildings::{Building, BuildingType};
use crate::combat::Army;
use crate::kingdom::Kingdom;
use crate::resources::Resources;


// World Creation Part
pub fn save_world_to_system(kingdom: &Kingdom) -> bool {

    let kingdom_data_json = serde_json::to_string(kingdom).unwrap();

    let path = Path::new("./data/worldData.json");

    // If there is saved data, ask user for confirmation of deletion of previous world
    if path.exists() {
        // Return if user wants to create new world over previous or not so in case not, the menu can be shown
        // agian, and not ask for C / L / D straight away
        let create_world: bool = create_over_existing_world();

        if create_world {
            let _ = fs::write(path, kingdom_data_json); // File Writing set to varibe to avoid warning of unused return of fs::write()
            return true;
        }
        return false;
    }
    else {
        let _ = fs::write(path, kingdom_data_json); // File Writing set to varibe to avoid warning of unused return of fs::write()
        return true;
    }

}

fn create_over_existing_world() -> bool {
    loop {
        // Promt
        print!("{}", "An existing world has been found. Would you like you like to create a new world still? [Y / N]: ");
        io::stdout().flush().expect("failed to flush while asking if user wanted to create a new world over a saved 1");


        // User Choice
        let choice = get_choice_from_user();

        // Reaction to User Choice
        if      choice.to_lowercase() == String::from("y") || choice.to_lowercase() == String::from("yes") {
            return true;
        }
        else if choice.to_lowercase() == String::from("n") || choice.to_lowercase() == String::from("no") {
            return false;
        }
        else {
            println!("Not a valid choice!");
        }
    }
}


// World Loading Part
pub fn load_world_off_system() -> Kingdom{

    // GET THE KINGDOM DATA AS STRING
    let path = Path::new("./data/worldData.json");

    if path.exists() {
        let mut kingdom_data_file = File::open(path).unwrap();
    
        let mut kingdom_data_str = String::new();
        // Read kingdom data (JSON) and convert to String
        kingdom_data_file
            .read_to_string(&mut kingdom_data_str)
            .unwrap();
    
        // PARSE THE KINGDOM DATA AS STRING TO KINGDOM STRUCT
        let kingdom: Kingdom = serde_json::from_str(&kingdom_data_str).expect("Couldn't parse Kingdom data as string to JSON");

        return kingdom;
    }

    Kingdom { 
        name: String::new(), 
        king: String::new(),
        level: 0, 
        resources: Resources { wood: 0, stone: 0, food: 0, gold: 0 }, 
        buildings: vec![Building { building_type: BuildingType::Farm, level: 0}], 
        army: Army {  }, 
        turn: 0 
    }

}


// World Deletion Part
pub fn delete_saved_world_off_system() -> bool{
    let path = Path::new("./data/worldData.json"); 

    if path.exists() {
        let _ = fs::remove_file(path); // File deletion set to varibe to avoid warning of unused return of fs::remove_file()

        // Return true or false so if no world is saved, main menu fn would know  what to do
        return true;
    }

    // Return true or false so if no world is saved, main menu fn would know  what to do
    return false;
}

fn get_choice_from_user() -> String{
    print!("{}", "What do you want to do?: ");
    // Flush stdout so the promt appears immediate before input
    io::stdout().flush().expect("Failed to flush stdout at menu");

    let mut user_choice: String = String::new();
    // Take input and save it
    io::stdin()
        .read_line(&mut user_choice)
        .expect("Failed to read user choice at menu");
    // Clean up the input
    let user_choice = user_choice.trim().to_lowercase();

    user_choice
}