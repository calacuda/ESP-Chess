use crate::biome;
use crate::enemies::{Enemy, MobSpawner};
use crate::items::Item;
use crate::level::Biome;
use kahuna::State;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ZoneType {
    Shelter,
    OpenPath,
    Coredor,
    Temple,
    Mountains,
    Sanctuary,
    TreasureRoom,
    BossEntry,
    Wall,
}

// TODO: change this to a zone trait with multiple structs that implement that trait

/// a single area in a level
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zone {
    pub name: Arc<str>,
    pub welcome: Arc<str>,
    pub desc: Arc<str>,
    pub feild_type: ZoneType,
    pub enemies: Option<MobSpawner>,
    pub loot: fn(&str) -> Option<Box<dyn Item>>,
    // pub hidden_loot: Arc<[Box<dyn Item>]>,
    // pub chest: Option<dyn Item>,
}

impl Zone {
    pub fn new(name: &str) -> Self {
        Zone {
            name: Arc::from(name),
            welcome: Arc::from(""),
            desc: Arc::from(""),
            feild_type: ZoneType::Wall,
            enemies: None,
            loot: |item_name| None,
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = Arc::from(name);
    }

    pub fn welcome(&mut self, welcome: &str) {
        self.welcome = Arc::from(welcome);
    }

    pub fn desc(&mut self, desc: &str) {
        self.welcome = Arc::from(desc);
    }

    pub fn feild_type(&mut self, feild: ZoneType) {
        self.feild_type = feild;
        // TODO: auto generate enemies, loot, and hidden_loot here.
    }

    pub fn set_item(&mut self, call_back: fn(&str) -> Option<Box<dyn Item>>) {
        self.loot = call_back;
    }

    // pub fn enemies(&mut self, enemies: Vec<Box<dyn Enemy>>) {
    //     self.enemies = Arc::from(enemies);
    // }

    // pub fn chest(&mut self, loot: Vec<dyn Item>) {
    //     self.chest = Some(Arc::from(loot));
    // }

    // pub fn hidden_loot(&mut self, loot: Vec<Box<dyn Item>>) {
    //     self.hidden_loot = Arc::from(loot);
    // }
}

impl State for Zone {
    fn entropy(&self) -> u32 {
        // TODO: figure out how the f to calculate this
        0
    }
}

pub struct BossRoom {
    /// the name of the lair (by default this is "The Lair of <Boss Name>")
    pub name: Arc<str>,
    /// the welcome message to be displaded when entering the zone
    pub welcome: Arc<str>,
    /// the description of the zone. gives more detail than the welcome banner.
    pub desc: Arc<str>,
    /// the boss monster
    pub boss: Option<Box<dyn Enemy>>,
    /// if the boss has lakies, they go here. not all bosses will have minions so this field is represented as an Option.
    pub minions: Option<Arc<[Box<dyn Enemy>]>>,
}

impl BossRoom {
    pub fn new(boss: Option<Box<dyn Enemy>>) -> Self {
        let default_name = Arc::from(format!("The {} Lair", biome!()).as_str());
        let default_welcome = Arc::from(
            "This battle may be your last, can you slay the final boss of this zone, and leave here with life and limb?"
        );
        let default_desc = Arc::from(format!("{default_name} has been the final escapade of many adventures. will you meet the same fate?").as_str());

        Self {
            name: default_name,
            welcome: default_welcome,
            desc: default_desc,
            boss,
            minions: None,
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = Arc::from(name);
    }

    pub fn welcome(&mut self, welcome: &str) {
        self.welcome = Arc::from(welcome);
    }

    pub fn desc(&mut self, desc: &str) {
        self.desc = Arc::from(desc);
    }

    pub fn set_minions(&mut self, minions: Option<Arc<[Box<dyn Enemy>]>>) {
        self.minions = minions;
    }

    pub fn set_boss(&mut self, boss: Box<dyn Enemy>) {
        self.boss = Some(boss);
    }

    pub fn kill_boss(&mut self) {
        self.boss = None;
    }
}
