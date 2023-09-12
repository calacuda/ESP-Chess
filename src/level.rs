use crate::enemies::Enemy;
use crate::zone::{BossRoom, Zone, ZoneType};
use anyhow::bail;
use kahuna::Space;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use strum_macros::EnumString;

type Coords = (u8, u8);

#[derive(Debug, Serialize, Deserialize, Copy, Clone, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Biome {
    Forest,
    Temple,
    Desert,
    Mountains,
    Ruins,
    Dungeon,
    Town,
}

impl fmt::Display for Biome {
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

#[macro_export]
macro_rules! biome {
    () => {{
        use std::str::FromStr;

        let Ok(b) = Biome::from_str(env!("XORK_BIOME")) else { panic!("The XORK_BIOME env variable was set incorrectly. it must be one of; 'forest', 'temple', 'desert', 'mountains', 'ruins', 'dungeon', or 'town'. (case insensitive)"); };

        b
    }};
}

pub struct Level {
    pub zones: Vec<Vec<Zone>>,
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
        let dim: (u8, u8) = (rng.gen_range(10..25), rng.gen_range(10..25));

        let zones: Vec<Vec<Zone>> = (0..=dim.0)
            .into_iter()
            .map(|i| {
                    (0..=dim.1)
                        .into_iter()
                        .map(|j| {
                            if (i, j) == (0, 0) {
                                let mut zone = Zone::new("Warp Zone");
                                zone.welcome(
                                    "You feel an emence evil eminating from a warp portal on the ground.",
                                );
                                zone.desc("No mob dares to near this place.");
                                zone.feild_type(ZoneType::BossEntry);

                                zone
                            } else {
                                Zone::new("")
                            }
                        })
                        .collect::<Vec<Zone>>()
                })
                .collect();

        Level {
            zones,
            dim,
            warp_in: (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1)),
            boss_portal: (0, 0),
            boss_room: BossRoom::new(boss),
            biome: biome!(),
        }
    }

    fn place_zone(
        &mut self,
        location: (u8, u8),
        name: &str,
        welcome: &str,
        description: &str,
        zone_type: ZoneType,
    ) {
        let mut zone = Zone::new(name);
        zone.welcome(welcome);
        zone.desc(description);
        zone.feild_type(zone_type);
    }

    /// generates the level
    pub fn init_level(&mut self) -> anyhow::Result<()> {
        // TODO: write this function

        // 1. make self.warp_in a sanctuary. no mobs, no loot, just a sign with imformation on it
        //    and a warp portal
        // 2. then do wave function colapse form there.
        // 3. pick a locaiton for the Boss room portal. (this locaiton must be accesable from the
        //    main game and have and present the user with a warning before entering. that way they
        //    don't accidentally stumble into the boss room and die instantly.)

        self.place_zone(
            self.warp_in,
            &format!("{} Sanctuary", biome!()),
            "Welcome to Xork!", 
            &format!("You find your self in a {}. Don't worry though, while monsters lay beyond ready to attack, you're save here. Now go forth and seize your destiny.", biome!()),
            ZoneType::Sanctuary
        );

        let mut i = 1;

        loop {
            self.wave_func_colapse()?;

            if self.traversable() {
                break;
            } else if i == 5 {
                bail!("tried wave function colapse unsuccessfully too many times.")
            }

            i += 1;
        }

        Ok(())
    }

    /// returns true if the player can make it from the warp in to the boss battle. else returns
    /// false.
    fn traversable(&mut self) -> bool {
        // TODO: implement A* from the warp_in sanctuary to the boss_portal. to ensure its at least
        // playable.
        false
    }

    /// chooses zones based on the biome and surrounding zones
    fn wave_func_colapse(&mut self) -> anyhow::Result<()> {
        // TODO: implement

        Ok(())
    }
}

impl Index<Coords> for Level {
    type Output = Zone;

    fn index(&self, index: Coords) -> &Self::Output {
        &self.zones[index.1 as usize][index.0 as usize]
    }
}

impl IndexMut<Coords> for Level {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        &mut self.zones[index.1 as usize][index.0 as usize]
    }
}

pub enum Directions {
    North,
    South,
    East,
    West,
}

impl Space<Zone> for Level {
    type Coordinate = Coords;
    type CoordinateDelta = Directions;

    fn coordinate_list(&self) -> Box<[Self::Coordinate]> {
        Box::from(
            (0..self.dim.1)
                .map(|i| (0..self.dim.0).map(move |j| (i, j)))
                .flatten()
                .collect::<Vec<_>>(),
        )
    }

    fn neighbors(
        &self,
        coord: Self::Coordinate,
        neighbor_directions: &[Self::CoordinateDelta],
        neighbors: &mut [Option<Self::Coordinate>],
    ) {
    }
}
