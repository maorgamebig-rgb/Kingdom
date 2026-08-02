use std::{thread, time};
use std::io::{self, Write};
use owo_colors::OwoColorize;


use crate::kingdom::Kingdom;
use crate::world::{create_world, GameState};
use crate::save_system::{load_world_off_system, delete_saved_world_off_system};


// Enum of continue and exit so I can tell the menu loop if it should break after doing a commend or continue after invalid commend
enum MenuAction {
    Continue,
    Exit,
}


pub fn main_menu() -> GameState{
    loop {
        // Main Menu
        title();
        objective();
        avalibe_commends();


        // Sleep for 1 sec
        let second = time::Duration::from_millis(1000);
        thread::sleep(second);
        
        // World creation / load / deletion loop
        loop {
            let user_action: String = take_action_from_user();

            match do_user_commend(&user_action) {
                MenuAction::Continue => continue,
                MenuAction::Exit     => break,
            }
        }
        
        let kingdom: Kingdom = load_world_off_system();

        // Sleep for 0.5 sec
        let half_sec = time::Duration::from_millis(500);
        thread::sleep(half_sec);

        // Verify if kingdom is a real saved kingdom or a placeholder bec game tried to load kingdom but there wasnt any kingdom saved
        if kingdom.level == 0 {
            println!("{}", "\nNo Saved Kingdom! Loading Failed!".red());

            // Sleep for 1 sec
            let second = time::Duration::from_millis(1000);
            thread::sleep(second);

            continue;
        }

        println!("{}", "World loaded successfully!".green());
        return GameState{ kingdom };
    }
}


// BASIC MENU DISPLAY
fn title() {
    println!("\n==== KINGDOM MADE BY DUMBASS ====");
    println!("{:^33}", "by alex & maor\n");
}

fn objective() {
    println!("Objective: Make your kingdom the best there is!");
    println!("{:>11}{}","", "Collect resources, Build buildings, Recruit an army");
    println!("{:>11}{}","", "And strive for GREATNESS!\n");
}

fn avalibe_commends() {
    println!("{}", "Type H for commends\n".blue());

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

            // Return if world was created or not so in case not, the menu can be shown
            // agian, and not ask for C / L / D straight away
            let world_created: bool = create_world();

            // Sleep for 1 sec
            let second = time::Duration::from_millis(1000);
            thread::sleep(second);

            if world_created == false {
                title();
                objective();
                avalibe_commends();


                return MenuAction::Continue;
            }

            MenuAction::Exit
        }
        "l" => {

            // Doesnt do anything bec world will always be loaded at the end of the main menu fn, so just a loading message will be shown when user chooses to load world 
            println!("{}", "Loading world...".green());

            MenuAction::Exit
        }
        "d" => {
            
            // Sleep for 0.5 sec
            let half_sec = time::Duration::from_millis(500);
            thread::sleep(half_sec);

            print!("{}", "\nAre you sure you want to delete the saved world? [Y / N]: ".red());
            // Flush stdout so the promt appears immediate before input
            io::stdout().flush().expect("Failed to flush stdout at menu");

            let mut user_choice: String = String::new();
            // Take input and save it
            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read user choice at menu");
            // Clean up the input
            let user_choice = user_choice.trim().to_lowercase();

            if      user_choice.to_lowercase() == String::from("y") || user_choice.to_lowercase() == String::from("yes") {

                if delete_saved_world_off_system() {
                    println!("{}", "World deleted successfully".green());    
                }

            }
            else if user_choice.to_lowercase() == String::from("n") || user_choice.to_lowercase() == String::from("no") {
                println!("{}", "\nWorld not deleted".green());
            }
            else {
                println!("Not a valid choice!");
            }

            // Sleep for 1 sec
            let second = time::Duration::from_millis(1000);
            thread::sleep(second);

            title();
            objective();
            avalibe_commends();

            MenuAction::Continue
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