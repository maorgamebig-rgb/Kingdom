mod menu;
use menu::main_menu;
mod world;
mod kingdom;
mod resources;
mod combat;
mod buildings;
mod save_system;

fn main() {
   // Run main menu (world creation / load / delete)
   main_menu(); 
}