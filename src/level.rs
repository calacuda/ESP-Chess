use crate::enemies::Enemy;
use crate::world::BIOME_TYPE;
use crate::zone::{BossRoom, Zone};
use core::fmt;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::Arc;
// use env_extract::{ConfigStruct, EnvVar};

// macro_rules! get_biome {
//     ("forest") => Biome::Forest;
//     ("temple") => Biome::Temple;
//     ("desert") => Biome::Desert;
//     ("mountains") => Biome::Mountains;
//     ("ruins") => Biome::Ruins;
//     ("dungeon") => Biome::Dungeon;
//     ("town") => Biome::Town;
// }

// #[macro_export]
// macro_rules! get_biome {
//     ($env_var: tt) => {
//         Biome::from(env!($env_var))
//     };
// }

#[macro_export]
macro_rules! get_biome {
    ($env_var: tt) => {
        match env!($env_var).to_lowercase().as_str() {
            "forest" => Biome::Forest,
            "temple" => Biome::Temple,
            "desert" => Biome::Desert,
            "mountains" => Biome::Mountains,
            "ruins" => Biome::Ruins,
            "dungeon" => Biome::Dungeon,
            "town" => Biome::Town,
            _ => panic!("The XORD_BIOME env variable was set incorrectly. it must be one of; 'forest', 'temple', 'desert', 'mountains', 'ruins', 'dungeon', or 'town'. (case insensitive)"),
        }
    };
}

#[derive(Debug)]
// #[derive(EnvVar)]
// #[var_name = "XORK_BIOME"]
// #[panic_on_invalid]
// #[case(convert = "lowercase")]
#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum Biome {
    Forest,
    Temple,
    Desert,
    Mountains,
    Ruins,
    Dungeon,
    Town,
}

impl From<&str> for Biome {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "forest" => Biome::Forest,
            "temple" => Biome::Temple,
            "desert" => Biome::Desert,
            "mountains" => Biome::Mountains,
            "ruins" => Biome::Ruins,
            "dungeon" => Biome::Dungeon,
            "town" => Biome::Town,
            _ => panic!("The XORD_BIOME env variable was set incorrectly. it must be one of; 'forest', 'temple', 'desert', 'mountains', 'ruins', 'dungeon', or 'town'. (case insensitive)"),
        }
    }
}

impl Display for Biome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Biome::Forest => write!(f, "Forest"),
            Biome::Temple => write!(f, "Temple"),
            Biome::Desert => write!(f, "Desert"),
            Biome::Mountains => write!(f, "Mountains"),
            Biome::Ruins => write!(f, "Ruins"),
            Biome::Dungeon => write!(f, "Dungeon"),
            Biome::Town => write!(f, "Town"),
        }
    }
}

pub struct Level {
    pub zones: Arc<[Zone]>,
    /// the dimensions of the level
    pub dim: (u8, u8),
    /// where the players spawns in
    pub warp_in: (u8, u8),
    /// the location of the portal to the boss's lair.
    pub boss_portal: (u8, u8),
    /// the boss room. the only way to access this room is via the warp portal in the boss_portal
    /// zone
    pub boss_room: BossRoom,
    /// defines the biome type of the level
    pub biome: Biome,
}

impl Level {
    pub fn new(boss: Box<dyn Enemy>) -> Self {
        let mut rng = rand::thread_rng();
        let dim = (rng.gen(), rng.gen());

        Level {
            zones: Arc::new([]),
            dim,
            warp_in: (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1)),
            boss_portal: (0, 0),
            boss_room: BossRoom::new(boss),
            biome: *BIOME_TYPE,
        }
    }

    /// generates the level
    pub fn init_level(&mut self) -> anyhow::Result<()> {
        // TODO: write this function

        // 1. make self.warp_in a sanctuary. no mobs, no loot, just a sign with imformation on it
        //    and a warp portal
        // 2. then do wave function colapse form there.
        // 3. pick a locaiton for the BBEG Boss room portal. (this locaiton must be accesable from the
        //    main game and have and present the user with a warning before entering. that way they
        //    don't accidentally stumble into the boss room and die instantly.)

        self.wave_func_colapse()?;

        Ok(())
    }

    /// chooses zones based on the biome and surrounding zones
    fn wave_func_colapse(&mut self) -> anyhow::Result<()> {
        // TODO: implement
        Ok(())
    }
}
