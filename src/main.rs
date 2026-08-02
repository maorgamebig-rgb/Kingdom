mod menu;
use menu::main_menu;
mod world;
mod kingdom;
mod resources;
mod combat;
mod buildings;
mod save_system;
mod ui;
use ui::ui;

use crate::world::GameState;

fn main() {
   // Run main menu (world creation / load / delete)
   let game_state = main_menu();

   // Run a ui for the game
   ui(game_state).expect("Error in loading UI");
}