use serde::{Serialize, Deserialize};
use crate::resources::Resources;
use std::time::{Duration, Instant};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BuildingType {
    Barracks,
    Farm,
    Mine,
    LumberMill,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Buildings {
    pub building_type: BuildingType,
    pub level: u32,
}


impl Buildings {
    pub fn new(building_type: BuildingType, level: u32) -> Self {
        Buildings {
            building_type,
            level: 1,
        }
    }

    pub fn resource_output(&self) -> Resources {
        let mut output = Resources::default();
        
        match self.building_type {
            BuildingType::Farm => {
                output.food = 10 * self.level;
            }
            BuildingType::Mine => {
                output.stone = 15 * self.level;
                output.gold = 5 * self.level;
            }
            BuildingType::LumberMill => {
                output.wood = 10 * self.level;
            }
            BuildingType::Barracks => {} // Barracks do not produce resources
        }
        output
    }

    pub fn upgrade_cost(&self) -> u32 {
        match self.building_type {
            BuildingType::Farm => 50 * self.level,
            BuildingType::Mine => 75 * self.level,
            BuildingType::Barracks => 100 * self.level,
            BuildingType::LumberMill => 150 * self.level,
        }
    }

    pub fn upgrade(&mut self) {
         self.level += 1;
    }
}