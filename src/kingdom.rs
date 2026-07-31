use crate::resources::Resources;
use crate::combat::Army;
use crate::buildings::Buildings;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};


#[derive(Serialize, Deserialize, Debug)]
pub struct Kingdom {
    pub name: String, // name of the kingdom duh
    pub king: String, // character name
    pub level: u32, // level of the kingdom, can be used to unlock new buildings or units.
    pub resources: Resources,
    pub buildings: Vec<Buildings>,
    pub army: Army,
    pub turn: u32, // or tick count
    #[serde(skip)]           // don't try to save/load an Instant
    pub last_tick: Option<Instant>,
}

impl Kingdom {
    const TICK_INTERVAL: Duration = Duration::from_secs(30);
    
    pub fn tick_resources(&mut self) {
        let now = Instant::now();
        let last = self.last_tick.get_or_insert(now);
        let elapsed = now.duration_since(*last);

        let ticks_passed = elapsed.as_secs() / Self::TICK_INTERVAL.as_secs();

        for _ in 0..ticks_passed {
            self.tick_resources();
        }

        if ticks_passed > 0 {
            self.last_tick = Some(now);
        }
    }
}