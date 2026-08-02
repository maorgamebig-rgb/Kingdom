use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Army {
    pub knights: u32,
    pub archers: u32,
}