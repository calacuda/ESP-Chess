use crate::enemies::Enemy;
use crate::items::Item;
use crate::I2cAdr;
use std::sync::Arc;

pub struct Zone {
    pub name: Arc<str>,
    pub welcome: Arc<str>,
    pub desc: Arc<str>,
    pub enemies: Arc<[Box<dyn Enemy>]>,
    pub loot: Arc<[Box<dyn Item>]>,
    pub hidden_loot: Arc<[Box<dyn Item>]>,
    pub maker_adr: I2cAdr,
}

impl Zone {
    pub fn new() -> Self {
        // TODO: write Zone maker
        Zone {
            name: Arc::from(""),
            welcome: Arc::from(""),
            desc: Arc::from(""),
            enemies: Arc::from([]),
            loot: Arc::from([]),
            hidden_loot: Arc::from([]),
            maker_adr: 0,
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = Arc::from(name);
    }
}
