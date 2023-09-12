use crate::{
    biome,
    enemies::boss::Boss,
    level::{Biome, Level},
};
use anyhow::Result;
use log::info;

pub struct World {
    /// the levels that the player has access to (both the players own level and all those they have
    /// traded for)
    pub levels: Vec<Level>, // TODO: limit this size based on mem-size and how much a single level
                            // takes up. make it an array of size (mem-size/level-size).
}

impl World {
    pub fn new() -> Self {
        Self {
            levels: Vec::with_capacity(10),
        }
    }

    /// generates the players biome using wave form generation
    pub fn generate(&mut self) -> Result<()> {
        let biome = biome!();

        info!("generating a {biome} biome...");

        let boss = Boss::new(biome);
        let mut level = Level::new(Box::from(boss));
        level.init_level()?;
        self.levels.push(level);

        info!("biome created.");

        Ok(())
    }
}
