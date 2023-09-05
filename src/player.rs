use std::sync::Arc;

use rand::seq::SliceRandom;

use crate::battle_logic::{BattleState, Spell};

const DEFAULT_NAMES: [&str; 10] = [
    "Calacuda",    // me
    "Eragon",      // main character from "Eragon"
    "Percy",       // main character from "Percy Jackson & the Olympians"
    "Peter",       // main character from "Peter and the Starcatcher"
    "Artemis",     // main character from "Artemis Fowl"
    "Christopher", // Christopher Paolini (author of "Eragon")
    "Eoin",        // Eoin Colfer (author of "Artemis Fowl")
    "Lloyd",       // Lloid Alexander (author of "The Prydian Chronicles" and translator of
    // Jean-Paul Sartre)
    "Rick",  // Rick Riordan (author of "Percy Jackson & the Olympians")
    "Barry", // Barry Pearcon (author of "Peter and the Starcatcher")
];

/// describes a status effect to be applied to players, NPCs, or Enemies. this trait allows for
/// buffs to be aplyed and can wear off with time or take effect for ever.
pub trait StatusEffect {
    /// returns what the effect should be referred to as when displayed to the player. (poison,
    /// slowness, burn, etc)
    fn display_name(&mut self) -> Arc<str>;

    /// returns true if the affect is still affecting the entity
    fn in_effect(&mut self) -> bool;

    /// returns the buff that should be applied to the entity
    fn get_buff(&mut self) -> Buff;

    /// a generic step function to update the internal state of the effect. used to handle generic
    /// house keeping for the effect, and to do any calculation the effect needs to do. for
    /// example: if the affect of the affect gets more/less sevear over time, do that math here.
    fn step(&mut self, battle: BattleState) -> anyhow::Result<()>;
}

/// how a buff should impact stats.
pub enum BuffType {
    /// indicates the ussage of a log function to generate the buff (better at lower levels).
    Log(i8),
    /// indicates the ussage of a exponential function to generate the buff (better at higher levels).
    Exp(i8),
    /// indicates that the buff (the f32) should be multitplied by the stat)
    Multiplier(f32),
    /// indicates that the buff (the i16) should be **ADDED** to the stat. if you want a constant
    /// debuff, use a negative number.
    Const(i16),
}

/// buff/debuff for players & enemies
pub struct Buff {
    pub hp: BuffType,
    pub str: BuffType,
    pub mg_str: BuffType,
    pub def: BuffType,
    pub mg_def: BuffType,
    pub speed: BuffType,
}

pub struct Stats {
    /// how much health the player has
    pub hp: u8,
    /// strength. ie, how much physical damage the player does not accounting for buffs from
    /// spells, potions, or status effects.
    pub str: u8,
    /// magic strength. ie, how much damage magic attacks *FROM* this entity will do.
    pub mg_str: u8,
    /// deffence. ie, how much physical damage resistance the entity has.
    pub def: u8,
    /// magic deffence. ie, how much the entity resists magical damage.
    pub mg_def: u8,
    /// speed. ie, how fast the entity is, determines attack order and frequency
    pub speed: u8,
}

/// where does the entity hold/wear this Equipment
pub enum EquipType {
    Helmet,
    Body,
    Pants,
    Hands,
    FullBody,
    Pendant,
    Title,
    Ring,
    WeaponHand,
    OffHand,
}

/// equipment for the player, NPCs, or Enemies
pub struct Equipment {
    /// where does the entity hold/wear this Equipment
    pub kind: EquipType,
    /// the buff to be applied to the entity who equips this Equipment
    pub buff: Buff,
}

pub struct Equipped {
    pub helmet: Option<Equipment>,
    pub body: Option<Equipment>,
    pub pants: Option<Equipment>,
    pub hands: Option<Equipment>,
    pub full_body: Option<Equipment>,
    pub pendant: Option<Equipment>,
    pub title: Option<Equipment>,
    pub ring: Option<Equipment>,
    pub main_weapon: Option<Equipment>,
    pub off_hand: Option<Equipment>,
}

impl Equipped {
    pub fn new() -> Self {
        Self {
            helmet: None,
            body: None,
            pants: None,
            hands: None,
            full_body: None,
            pendant: None,
            title: None,
            ring: None,
            main_weapon: None,
            off_hand: None,
        }
    }
}

// TODO: implement Item trait for Equipment

// TODO: finish player struct.

/// represents a player character
pub struct Player {
    /// the name of the player character
    pub name: Arc<str>,
    /// the players score
    pub score: u16, // could likely be a u8 but using u16 to be safe.
    // the player has access to all items just some have a count of zero. but this is still here just in case
    // inventory: Vec<Box<dyn Item>>,
    /// the spells the player knows.
    pub spells: Vec<Box<dyn Spell>>,
    /// the players current stats.
    pub stats: Stats,
    /// the equipment the player has equipped.
    pub gear: Equipped,
    /// anny buffs that have been aplyed by spells, potions, etc
    pub bufs: Vec<Buff>,
    /// a list of status effects affectign the player.
    pub status: Vec<Box<dyn StatusEffect>>,
}

impl Player {
    pub fn new(name: Option<&str>) -> Self {
        let mut rng = rand::thread_rng();

        Player {
            name: name
                .unwrap_or(&DEFAULT_NAMES.choose(&mut rng).unwrap_or(&DEFAULT_NAMES[0]))
                .into(),
            score: 0,
            spells: Vec::new(),
            stats: Stats {
                hp: 20,
                str: 5,
                mg_str: 1,
                def: 4,
                mg_def: 2,
                speed: 4,
            },
            gear: Equipped::new(),
            bufs: Vec::new(),
            status: Vec::new(),
        }
    }

    pub fn name(&mut self, name: &str) {
        self.name = Arc::from(name);
    }
}

// TODO: impl battle_logic::Battlable for Player
