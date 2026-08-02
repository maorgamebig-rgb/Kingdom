use std::io::{self, Write};
use owo_colors::OwoColorize;


use crate::kingdom::Kingdom;
use crate::resources::Resources;
use crate::combat::Army;


use crate::save_system::save_world_to_system;


pub struct GameState {
    pub kingdom: Kingdom,
    // later: map, ai, etc
}


// Return if world was created or not so in case not, the menu can be shown
// agian, and not ask for C / L / D straight away
pub fn create_world() -> bool {

    let mut kingdom = init_kingdom();

    world_creation_finished_message(&mut kingdom);

    let created_world: bool = save_world_to_system(&kingdom);

    created_world
}

fn init_kingdom() -> Kingdom {
    let kingdom: Kingdom = Kingdom {
        name: String::new(),
        king: String::new(),
        level: 1,
        resources: Resources {
            wood: 250,
            stone: 250,
            food: 250,
            gold: 250
        },
        buildings: Vec::new(),
        army: Army { knights: 3, archers: 0},
        turn: 0,
        last_tick: None,
    };

    kingdom
}

fn world_creation_finished_message(kingdom: &mut Kingdom) {
    kingdom.king = take_username_from_user();

    println!("The king {} has been born!\n", kingdom.king);

    kingdom.name = take_kingdom_name_from_user();

    let final_message = format!("The kingdom {} is under {}'s control!\n", kingdom.name, kingdom.king);
    
    println!("{}", final_message.yellow());

    println!("World created!\nYour adventure as a king has just started\n");
}

fn take_username_from_user() -> String{

    print!("{}", "\nHow should you be called?: ".green());
    // Flush stdout so the promt appears immediate before input
    io::stdout().flush().expect("Failed to flush stdout druing king's name selection");

    let mut username: String = String::new();
    // Take input and save it
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read user action druing king's name selection");
    // Clean up the input
    let username = username.trim().to_lowercase();

    username

}

fn take_kingdom_name_from_user() -> String{

    print!("{}", "How should the kingdom be called?: ".green());
    // Flush stdout so the promt appears immediate before input
    io::stdout().flush().expect("Failed to flush stdout druing kingdom's name selection");

    let mut kingdome_name: String = String::new();
    // Take input and save it
    io::stdin()
        .read_line(&mut kingdome_name)
        .expect("Failed to read user action druing kingdom's name selection");
    // Clean up the input
    let kingdome_name = kingdome_name.trim().to_lowercase();

    kingdome_name
    
}
