use std::path::Path;
use std::fs;
use std::io::{self, Write};


use crate::kingdom::Kingdom;


pub fn save_world_to_system(kingdom: &Kingdom) -> bool {

    let json_kingdom = serde_json::to_string(kingdom).unwrap();

    let path = Path::new("./data/worldData.json");

    if path.exists() {
        let create_world: bool = create_over_existing_world();

        if create_world {
            let _ = fs::write(path, json_kingdom); // File Writing set to varibe to avoid warning of unused return of fs::write()
            return true;
        }
        return false;
    }
    else {
        let _ = fs::write(path, json_kingdom); // File Writing set to varibe to avoid warning of unused return of fs::write()
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
            println!("Choice not understood!");
        }
    }
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