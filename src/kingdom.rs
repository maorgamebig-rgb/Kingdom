use crate::resources::Resources;
use crate::combat::Army;
use crate::buildings::Buildings;

pub struct Kingdom {
    pub name: String, // name of the kingdom duh
    pub king: String, // character name
    pub level: u32, // level of the kingdom, can be used to unlock new buildings or units.
    pub resources: Resources,
    pub buildings: Vec<Buildings>,
    pub army: Army,
    pub turn: u32, // or tick count
}