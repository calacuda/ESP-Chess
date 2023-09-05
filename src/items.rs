use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub trait Item {
    // TODO: write Item trait
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ItemKey {
    // level: u8,
    // loc: (u8, u8),
    name: Arc<str>,
    /// gives a max of 256 for each item
    inst: u8,
    // maker_adr: I2cAdr,
}
