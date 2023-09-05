use crate::enemies::Enemy;
use crate::items::ItemKey;
use std::sync::Arc;

pub enum ZoneType {
    Forest,
    Path,
    Coredor,
    Temple,
    Desert,
    Marsh,
    Mountains,
    Sanctuary,
    TreasureRoom,
}

// TODO: make zone a trait. that way each zone can have unique behavior

/// a single area in a level
pub struct Zone {
    pub name: Arc<str>,
    pub welcome: Arc<str>,
    pub desc: Arc<str>,
    pub feild_type: ZoneType,
    // pub enemies: Arc<[Box<dyn Enemy>]>,
    // pub loot: Arc<[Box<dyn Item>]>,
    // pub hidden_loot: Arc<[Box<dyn Item>]>,
    pub chest: Option<Arc<[ItemKey]>>,
}

impl Zone {
    pub fn new(name: &str) -> Self {
        Zone {
            name: Arc::from(name),
            welcome: Arc::from(""),
            desc: Arc::from(""),
            feild_type: ZoneType::Sanctuary,
            // enemies: Arc::from([]),
            chest: None, // Arc::from([]),
                         // hidden_loot: Arc::from([]),
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
    }

    // pub fn enemies(&mut self, enemies: Vec<Box<dyn Enemy>>) {
    //     self.enemies = Arc::from(enemies);
    // }

    pub fn chest(&mut self, loot: Vec<ItemKey>) {
        self.chest = Some(Arc::from(loot));
    }

    // pub fn hidden_loot(&mut self, loot: Vec<Box<dyn Item>>) {
    //     self.hidden_loot = Arc::from(loot);
    // }
}

// impl I2cCom<Zone> for Zone {
//     fn encode(&self) -> anyhow::Result<Vec<u8>> {
//         let json = serde_json::to_string(self)?;
//
//         Ok(Vec::from(json.as_bytes()))
//     }
//
//     fn decode_from(&self, data: &[u8]) -> anyhow::Result<Zone> {
//         let json = String::from_utf8_lossy(data);
//
//         Ok(serde_json::from_str(&json)?)
//     }
// }

pub struct BossRoom {
    /// the name of the lair (by default this is "The Lair of <Boss Name>")
    pub name: Arc<str>,
    /// the welcome message to be displaded when entering the zone
    pub welcome: Arc<str>,
    /// the description of the zone. gives more detail than the welcome banner.
    pub desc: Arc<str>,
    /// the boss monster
    pub boss: Box<dyn Enemy>,
    /// if the boss has lakies, they go here. not all bosses will have minions so this field is represented as an Option.
    pub minions: Option<Arc<[Box<dyn Enemy>]>>,
}

impl BossRoom {
    pub fn new(boss: Box<dyn Enemy>) -> Self {
        let default_name = Arc::from(format!("The Lair of {}", boss.get_name()).as_str());
        let default_welcome = Arc::from(
            format!(
            "This battle may be your last, can you slay \"{}\", and leave here with life and limb?",
            boss.get_name()
        )
            .as_str(),
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
}
