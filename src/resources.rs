use serde::{Serialize, Deserialize};
use std::ops::AddAssign;


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct Resources {
    pub wood: u32,
    pub stone: u32,
    pub food: u32,
    pub gold: u32,
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        self.wood += other.wood;
        self.stone += other.stone;
        self.food += other.food;
        self.gold += other.gold;
    }
}