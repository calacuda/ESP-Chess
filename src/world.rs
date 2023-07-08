// use crate::level::Level;
use crate::I2cAdr;
use std::sync::Arc;

/// a place holder level to be used stores information about how to get the actual level
pub struct LevelSkel {
    /// the dimensions of the level
    dim: (u8, u8),
    /// the location of the warp portal to the previous level.
    warp_prev: Option<(u8, u8)>,
    /// the location of the warp portal to the next level.
    warp_next: Option<(u8, u8)>,

    floor: u8,

    maker_adr: I2cAdr,
}

impl LevelSkel {
    fn new(floor: u8, maker: I2cAdr) -> Self {
        let (max_x, max_y) = rand_pair(255, 255);

        LevelSkel {
            dim: (max_x, max_y),
            warp_prev: if floor > 0 {
                Some(rand_pair(max_x, max_y))
            } else {
                None
            },
            warp_next: if floor < 255 {
                Some(rand_pair(max_x, max_y))
            } else {
                None
            },
            floor,
            maker_adr: maker,
        }
    }
}

fn rand_pair(max_x: u8, max_y: u8) -> (u8, u8) {
    let mut res = (rand::random(), rand::random());

    // weighted random (insert sparkles & confetti here).
    if res.0 > max_x {
        res.0 -= max_x;
    }

    if res.1 > max_y {
        res.1 -= max_y;
    }

    res
}

pub struct World {
    pub levels: Arc<[LevelSkel]>,
    pub n_levels: u8,
}

impl World {
    pub fn new(workers: &[I2cAdr]) -> Self {
        let n_levels: u8 = rand::random();
        // let mut levels = Vec::with_capacity(n_levels.into());

        // for i in 0..n_levels {
        //     levels.push(i);
        // }

        let skels: Vec<LevelSkel> = (0..n_levels)
            //.collect()
            //.iter()
            .map(|i| LevelSkel::new(i, workers[i as usize % workers.len()]))
            .collect();

        Self {
            levels: skels.into(),
            n_levels,
        }
    }

    /// generates the levels in parallel
    pub fn generate(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
