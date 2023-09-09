use crate::{
    get_biome,
    level::{Biome, Level},
};
use anyhow::Result;
use lazy_static::lazy_static;
use log::info;

lazy_static! {
    pub static ref BIOME_TYPE: Biome = get_biome!("XORK_BIOME");
}

pub struct World {
    /// the levels that the player has access to (both the players own level and all those they have
    /// traded for)
    pub levels: Vec<Level>,
}

impl World {
    pub fn new() -> Self {
        Self {
            levels: Vec::with_capacity(10),
        }
    }

    /// generates the players biome
    pub fn generate(&mut self) -> Result<()> {
        info!("generating a {} Biome", *BIOME_TYPE);

        let boss = todo!("see below");
        // TODO: make a mod folder for enemies and then make a boss enemy that implents the enemy
        // trait

        let mut level = Level::new(Box::from(boss));
        level.init_level()?;
        self.levels[0] = level;

        Ok(())
    }
}
