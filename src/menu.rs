use std::io::{self, Write};
use owo_colors::OwoColorize;


use crate::world::{self, create_world};


// Enum of continue and exit so I can tell the menu loop if it should break after doing a commend or continue after invalid commend
enum MenuAction {
    Continue,
    Exit,
}


pub fn main_menu() {
    // Main Menu
    title();
    objective();
    avalibe_commends();
    
    loop {
        let user_action: String = take_action_from_user();

        match do_user_commend(&user_action) {
            MenuAction::Continue => continue,
            MenuAction::Exit     => break,
        }
    }
    
}


// BASIC MENU DISPLAY
fn title() {
    println!("==== KINGDOM MADE BY DUMBASS ====");
    println!("{:^33}", "by alex & maor\n");
}

fn objective() {
    println!("Objective: Make your kingdom the best there is!");
    println!("{:>11}{}","", "Collect resources, Build buildings, Recruit an army");
    println!("{:>11}{}","", "And strive for GREATNESS!\n");
}

fn avalibe_commends() {
    println!("{}", "Type H for commends\n".blue());

    //TODO create a storage system for worlds
    println!("{}", "C - create new world".yellow());
    println!("{}", "L - load world".yellow());
    println!("{}", "D - delete world".yellow());
}


// USER COMMEND
fn take_action_from_user() -> String{
    print!("{}", "What do you want to do?: ".green());
    // Flush stdout so the promt appears immediate before input
    io::stdout().flush().expect("Failed to flush stdout at menu");

    let mut user_action: String = String::new();
    // Take input and save it
    io::stdin()
        .read_line(&mut user_action)
        .expect("Failed to read user action at menu");
    // Clean up the input
    let user_action = user_action.trim().to_lowercase();

    user_action
}

fn do_user_commend(commend: &str) -> MenuAction {
    match commend {
        "c" => {
            world::create_world();

            MenuAction::Exit
        }
        "l" => {
            //TODO function of deleting a world

            //TEMP DELETE WHEN DID THAT
            println!("TEMP: loaded world!"); 

            MenuAction::Exit
        }
        "d" => {
            //TODO function of deleting a world

            //TEMP DELETE WHEN DID THAT
            println!("TEMP: deleted world!"); 

            MenuAction::Exit
        }
        "h" => {
            //TODO function for display all in-game commends

            //TEMP DELETE WHEN DID THAT
            println!("TEMP: showed in-game commends!"); 

            MenuAction::Exit
        }
        &_ => {
            println!("{}", "Not a valid choice. Enter choice agian.".red());
            MenuAction::Continue
        }
    }

}