use crate::resources::Resources;
use crate::combat::Army;
use crate::buildings::Building;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Kingdom {
    pub name: String, // name of the kingdom duh
    pub king: String, // character name
    pub level: u32, // level of the kingdom, can be used to unlock new buildings or units.
    pub resources: Resources,
    pub buildings: Vec<Building>,
    pub army: Army,
    pub turn: u32, // or tick count
}