use crate::enemies::Enemy;
use crate::zone::{BossRoom, Zone};
use anyhow::Result;
use kahuna::bitset_state::BitsetState;
use kahuna::square_grid::SquareGrid;
use kahuna::{collapse, set_rule::*, AllState};
use log::info;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use strum_macros::EnumString;

type Coords = (usize, usize);
type S = BitsetState<9>;

const SHELTER: S = S::state(0);
const OPEN_PATH: S = S::state(1);
const COREDOR: S = S::state(2);
const TEMPLE: S = S::state(3);
const MOUNTAINS: S = S::state(4);
const SANCTUARY: S = S::state(5);
const TRESURE_ROOM: S = S::state(6);
const BOSS_ENTRY: S = S::state(7);
const WALL: S = S::state(8);

const NORTH: (isize, isize) = (0, -1);
const SOUTH: (isize, isize) = (0, 1);
const WEST: (isize, isize) = (1, 0);
const EAST: (isize, isize) = (-1, 0);

type Grid = SquareGrid<S>;

fn get_rules() -> SetCollapseRule<S, Grid, UniformSetCollapseObserver> {
    SetCollapseRuleBuilder::new(UniformSetCollapseObserver)
        .allow(
            &SHELTER,
            &[
                (
                    NORTH,
                    OPEN_PATH | COREDOR | MOUNTAINS | SANCTUARY | BOSS_ENTRY | WALL,
                ),
                (
                    SOUTH,
                    OPEN_PATH | COREDOR | MOUNTAINS | SANCTUARY | BOSS_ENTRY | WALL,
                ),
                (
                    EAST,
                    OPEN_PATH | COREDOR | MOUNTAINS | SANCTUARY | BOSS_ENTRY | WALL,
                ),
                (
                    WEST,
                    OPEN_PATH | COREDOR | MOUNTAINS | SANCTUARY | BOSS_ENTRY | WALL,
                ),
            ],
        )
        .allow(
            &OPEN_PATH,
            &[
                (
                    NORTH,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY,
                ),
                (
                    SOUTH,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY,
                ),
                (
                    EAST,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY,
                ),
                (
                    WEST,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY,
                ),
            ],
        )
        .allow(
            &COREDOR,
            &[
                (
                    NORTH,
                    OPEN_PATH | COREDOR | TEMPLE | SANCTUARY | TRESURE_ROOM | BOSS_ENTRY,
                ),
                (
                    SOUTH,
                    OPEN_PATH | COREDOR | TEMPLE | SANCTUARY | TRESURE_ROOM | BOSS_ENTRY,
                ),
                (
                    EAST,
                    OPEN_PATH | COREDOR | TEMPLE | SANCTUARY | TRESURE_ROOM | BOSS_ENTRY,
                ),
                (
                    WEST,
                    OPEN_PATH | COREDOR | TEMPLE | SANCTUARY | TRESURE_ROOM | BOSS_ENTRY,
                ),
            ],
        )
        .allow(
            &TEMPLE,
            &[
                (
                    NORTH,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | SANCTUARY | TRESURE_ROOM,
                ),
                (
                    SOUTH,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | SANCTUARY | TRESURE_ROOM,
                ),
                (
                    EAST,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | SANCTUARY | TRESURE_ROOM,
                ),
                (
                    WEST,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | SANCTUARY | TRESURE_ROOM,
                ),
            ],
        )
        .allow(
            &MOUNTAINS,
            &[
                (
                    NORTH,
                    OPEN_PATH | TEMPLE | MOUNTAINS | SANCTUARY | BOSS_ENTRY,
                ),
                (
                    SOUTH,
                    OPEN_PATH | TEMPLE | MOUNTAINS | SANCTUARY | BOSS_ENTRY,
                ),
                (
                    EAST,
                    OPEN_PATH | TEMPLE | MOUNTAINS | SANCTUARY | BOSS_ENTRY,
                ),
                (
                    WEST,
                    OPEN_PATH | TEMPLE | MOUNTAINS | SANCTUARY | BOSS_ENTRY,
                ),
            ],
        )
        .allow(
            &SANCTUARY,
            &[
                (
                    NORTH,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | TRESURE_ROOM | BOSS_ENTRY,
                ),
                (
                    SOUTH,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | TRESURE_ROOM | BOSS_ENTRY,
                ),
                (
                    EAST,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | TRESURE_ROOM | BOSS_ENTRY,
                ),
                (
                    WEST,
                    SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS | TRESURE_ROOM | BOSS_ENTRY,
                ),
            ],
        )
        .allow(
            &TRESURE_ROOM,
            &[
                (NORTH, SHELTER | COREDOR | TEMPLE),
                (SOUTH, SHELTER | COREDOR | TEMPLE),
                (EAST, SHELTER | COREDOR | TEMPLE),
                (WEST, SHELTER | COREDOR | TEMPLE),
            ],
        )
        // TODO: might need to remove this. bc this may cause there to be more then one boss
        // entry. and thats bad.
        .allow(
            &BOSS_ENTRY,
            &[
                (NORTH, SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS),
                (SOUTH, SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS),
                (EAST, SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS),
                (WEST, SHELTER | OPEN_PATH | COREDOR | TEMPLE | MOUNTAINS),
            ],
        )
        .allow(
            &WALL,
            &[
                (
                    NORTH,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY
                        | WALL,
                ),
                (
                    SOUTH,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY
                        | WALL,
                ),
                (
                    EAST,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY
                        | WALL,
                ),
                (
                    WEST,
                    SHELTER
                        | OPEN_PATH
                        | COREDOR
                        | TEMPLE
                        | MOUNTAINS
                        | SANCTUARY
                        | TRESURE_ROOM
                        | BOSS_ENTRY
                        | WALL,
                ),
            ],
        )
        .build()
}

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

/// the standard way to construct a new crate::level::Level struct.
pub struct LevelBuilder {
    // /// a 2D grid used to do wave function generation to generate the world
    // grid: Grid,
    /// the dimensions of the level
    dim: (isize, isize),
    /// where the players spawns in
    warp_in: (isize, isize),
    /// the location of the portal to the boss's lair.
    boss_portal: (isize, isize),
}

impl LevelBuilder {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let dim: (isize, isize) = (rng.gen_range(20..30), rng.gen_range(20..30));
        let warp_in = (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1));
        let mut boss_portal = (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1));

        while boss_portal == warp_in {
            boss_portal = (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1));
        }

        LevelBuilder {
            dim,
            warp_in,
            boss_portal,
        }
    }

    /// generates the level
    pub fn build(&self, boss: impl Enemy + 'static) -> Result<Level> {
        // TODO: write this function

        // 1. make self.warp_in a sanctuary. no mobs, no loot, just a sign with imformation on it
        //    and a warp portal
        // 2. then do wave function colapse form there.
        // 3. pick a locaiton for the Boss room portal. (this locaiton must be accesable from the
        //    main game and have and present the user with a warning before entering. that way they
        //    don't accidentally stumble into the boss room and die instantly.)

        // let mut i = 1;
        //
        // let template_level = self.wave_func_colapse()?;
        // let mut level = self.mk_level(template_level)?;
        //
        // while !self.traversable(&level) {
        //     template = self.wave_func_colapse()?;
        // }

        let mut level = loop {
            let template_level = self.wave_func_colapse()?;
            let level = self.mk_level(template_level);

            if self.traversable(&level) {
                break level;
            }
        };

        level.boss_room.set_boss(Box::new(boss));

        // info!("{:?}", level.zones);

        Ok(level)
    }

    fn mk_level(&self, template: Grid) -> Level {
        let mut zones: Vec<Box<[Zone]>> = Vec::with_capacity(self.dim.0 as usize);
        let blank: Vec<Zone> = Vec::with_capacity(self.dim.1 as usize);

        for y in 0..self.dim.0 {
            let mut row = blank.clone();

            for x in 0..self.dim.1 {
                // info!("{:?}", template[(x, y)]);

                let zone = match template[(x, y)] {
                    // TODO: make this create zones based on the state
                    SHELTER => Zone::new(&String::new()),
                    OPEN_PATH => Zone::new(&String::new()),
                    COREDOR => Zone::new(&String::new()),
                    TEMPLE => Zone::new(&String::new()),
                    MOUNTAINS => Zone::new(&String::new()),
                    SANCTUARY => Zone::new(&String::new()),
                    TRESURE_ROOM => Zone::new(&String::new()),
                    BOSS_ENTRY => Zone::new(&String::new()),
                    WALL => Zone::new(&String::new()),
                    _ => {
                        unreachable!("all zone types should be enumerated in this match statement.")
                    }
                };

                row.push(zone);
            }

            zones.push(Box::from(row));
        }

        Level {
            zones: Box::from(zones),
            dim: self.dim,
            warp_in: self.warp_in,
            boss_portal: self.boss_portal,
            boss_room: BossRoom::new(None),
            biome: biome!(),
        }
    }

    /// returns true if the player can make it from the warp in to the boss battle. else returns
    /// false.
    fn traversable(&self, level: &Level) -> bool {
        // TODO: preforms A* on level from the warp_in sanctuary to the boss_portal.
        // this ensures that the player can at least get to the boss room level is at
        // least playable.
        true
    }

    /// chooses zones based on the biome and surrounding zones
    fn wave_func_colapse(&self) -> Result<SquareGrid<S>> {
        // TODO: implement

        let rule = get_rules();
        let mut grid = SquareGrid::new(self.dim.0, self.dim.1, |_, _| S::all());

        collapse(&mut grid, &rule);
        // let mut neighbors = Vec::new();
        // self.zones
        //     .neighbors(self.warp_in, &[(0, 1)], &mut neighbors);
        //
        // info!("{:?}", neighbors);

        Ok(grid)
    }
}

pub struct Level {
    // pub zones: Vec<Vec<Zone>>,
    pub zones: Box<[Box<[Zone]>]>, // TODO: maybe make this a 1D array and just modulo when indexing is
    // needed?
    /// the dimensions of the level
    pub dim: (isize, isize),
    /// where the players spawns in
    pub warp_in: (isize, isize),
    /// the location of the portal to the boss's lair.
    pub boss_portal: (isize, isize),
    /// the boss room. the only way to access this room is via the warp portal in the boss_portal
    /// zone
    pub boss_room: BossRoom,
    /// defines the biome type of the level
    pub biome: Biome,
}

impl Level {
    // pub fn new(boss: Box<dyn Enemy>) -> Self {
    //     let mut rng = rand::thread_rng();
    //     let dim: (isize, isize) = (rng.gen_range(10..25), rng.gen_range(10..25));
    //
    //     let zones: Box<Box<[Zone]>> = Box::from((0..=dim.0)
    //         .into_iter()
    //         .map(|i| {
    //                 Box::from((0..=dim.1)
    //                     .into_iter()
    //                     .map(|j| {
    //                         if (i, j) == (0, 0) {
    //                             let mut zone = Zone::new("Warp Zone");
    //                             zone.welcome(
    //                                 "You feel an emence evil eminating from a warp portal on the ground.",
    //                             );
    //                             zone.desc("No mob dares to near this place.");
    //                             zone.feild_type(ZoneType::BossEntry);
    //
    //                             zone
    //                         } else {
    //                             Zone::new("")
    //                         }
    //                     })
    //                     .collect::<Vec<Zone>>())
    //             })
    //             .collect());
    //
    //     let start = (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1));
    //     let mut boss_portal = (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1));
    //
    //     while boss_portal == start {
    //         boss_portal = (rng.gen_range(0..dim.0), rng.gen_range(0..dim.1));
    //     }
    //
    //     Level {
    //         zones,
    //         dim,
    //         warp_in: start,
    //         boss_portal,
    //         boss_room: BossRoom::new(boss),
    //         biome: biome!(),
    //     }
    // }
}

impl Index<Coords> for Level {
    type Output = Zone;

    fn index(&self, index: Coords) -> &Self::Output {
        &self.zones[index.1][index.0]
    }
}

impl IndexMut<Coords> for Level {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        &mut self.zones[index.1][index.0]
    }
}

// pub enum Directions {
//     North,
//     South,
//     East,
//     West,
// }

// impl Space<Zone> for Level {
//     type Coordinate = Coords;
//     type CoordinateDelta = Directions;
//
//     fn coordinate_list(&self) -> Box<[Self::Coordinate]> {
//         Box::from(
//             (0..self.dim.1)
//                 .map(|i| (0..self.dim.0).map(move |j| (i, j)))
//                 .flatten()
//                 .collect::<Vec<_>>(),
//         )
//     }
//
//     fn neighbors(
//         &self,
//         coord: Self::Coordinate,
//         neighbor_directions: &[Self::CoordinateDelta],
//         neighbors: &mut [Option<Self::Coordinate>],
//     ) {
//     }
// }

// struct LocationRule {}
//
// impl kahuna::CollapseRule<Zone, SquareGrid<Zone>> for LocationRule {
//     fn neighbor_offsets(
//         &self,
//     ) -> Box<[<SquareGrid<Zone> as kahuna::Space<Zone>>::CoordinateDelta]> {
//         Box::from([(0, 1), (1, 0), (-1, 0), (0, -1)])
//     }
//
//     fn collapse(&self, cell: &mut Zone, neighbors: &[Option<Zone>]) {}
//
//     fn observe(&self, cell: &mut Zone, neighbors: &[Option<Zone>]) {}
// }
