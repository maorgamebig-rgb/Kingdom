use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BuildingType {
    Barracks,
    Farm,
    Mine,
    Tower,
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

    pub fn resource_output(&self) -> u32 {
        match self.building_type {
            BuildingType::Farm => 10 * self.level,
            BuildingType::Mine => 15 * self.level,
            BuildingType::Barracks => 0, // Barracks do not produce resources
            BuildingType::Tower => 0, // Towers do not produce resources
        }
    }

    pub fn upgrade_cost(&self) -> u32 {
        match self.building_type {
            BuildingType::Farm => 50 * self.level,
            BuildingType::Mine => 75 * self.level,
            BuildingType::Barracks => 100 * self.level,
            BuildingType::Tower => 150 * self.level,
        }
    }

    pub fn upgrade(&mut self) {
         self.level += 1;
    }
}