use crate::game::State;
use crate::Enemy;
use crate::Item;
use crate::Player;
use std::sync::Arc;

/// describes the target of an attack or spell.
pub enum Target {
    /// indicates an attack or spell that targets the self
    TheSelf,
    /// indicates an attack or spell that targets an enemy. which enemy is defined by the usize
    Enemy(usize),
}

pub enum BattleAction {
    /// indicates that the entity is attacking, holds a u8 of how much damage is being done
    Attack(u8),
    /// indicates the casting of a spell
    CastSpell((Box<dyn Spell>, Target)),
    /// indicates Fleeing (will end the battle)
    Flee,
    /// indicates the usage of an item
    UseItem((Box<dyn Item>, Target)),
}

// TODO: write an Ally trait

pub struct BattleState {
    /// the player character
    pub player: Player,
    /// the enemies the player is fighting
    pub enemies: Arc<[Box<dyn Enemy>]>, // maybe make this a hashset for easy removing on enemy death
    // allies: Arc<Box<dyn Ally>]>
    /// stores if the battle has concluded
    pub over: bool,
    /// the number of turns elapst sinc ethe beginning of the battle
    pub turn_n: u16,
}

impl State for BattleState {
    #[allow(unused_variables)]
    fn step(&mut self, cmd: &str, player: &Player) -> bool {
        // TODO: implement battle logic and battle command parsing
        self.over = self.over && !self.enemies.is_empty(); // should be idempotent

        self.is_done()
    }

    fn is_done(&self) -> bool {
        self.over
    }
}

pub trait Spell {
    fn new() -> Self
    where
        Self: Sized;
    fn cast(&mut self, state: &mut BattleState);
}

// TODO: write a Battlable trait to represnet entities that can enter a battle.
