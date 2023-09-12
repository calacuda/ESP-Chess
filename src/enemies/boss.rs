use crate::battle_logic::{BattleAction, BattleState};
use crate::enemies::{Enemy, Lifeness};
use crate::level::Biome;
use crate::player::Stats;
use std::sync::Arc;

pub struct Boss {
    name: Arc<str>,
    stats: Stats,
}

impl Boss {
    pub fn new(biome: Biome) -> Self {
        Self {
            name: Arc::from(format!("{biome} Boss")),
            stats: Stats {
                hp: 100,
                str: 15,
                mg_str: 15,
                def: 15,
                mg_def: 15,
                speed: 10,
            },
        }
    }
}

impl Enemy for Boss {
    /// returns the enemies name. (eg, "Orc", "Goblin #1", "Gregory the Destroyer of Worlds", etc)
    fn get_name(&self) -> Arc<str> {
        self.name.clone()
    }

    /// generates the move that the enemy will take
    fn get_move(&mut self, state: &mut BattleState) -> BattleAction {
        // TODO: write boss AI
        BattleAction::Attack(self.stats.str)
    }

    /// applies damage to the Enemy
    fn take_damage(&mut self, state: &mut BattleState) -> Lifeness {
        // TODO: implement damage taking

        Lifeness::Alive
    }
}
