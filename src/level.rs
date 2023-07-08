use crate::zone::Zone;
use std::sync::Arc;

pub struct Level {
    pub zones: Arc<[Zone]>,
    /// the dimensions of the level
    pub dim: (u8, u8),
    /// the location of the warp portal to the previous level.
    pub warp_prev: Option<(u8, u8)>,
    /// the location of the warp portal to the next level.
    pub warp_next: Option<(u8, u8)>,

    pub floor: u8,

    /// the I2C address of the maker esp32
    pub maker_adr: u8,
}

impl Level {
    pub fn new(i: u8) -> Self {
        // TODO: write level maker
        Level {
            zones: Arc::new([]),
            floor: i,
            maker_adr: 
        }
    }
}
