use crate::world::World;
use crate::Player;
use anyhow::bail;
use std::cell::RefCell;
use std::rc::Rc;

/// trait for a game state Explore, Battle, Interact, etc.
pub trait GameState {
    /// a generic step function to step through the state manchine, returns true when ready to exit
    /// (battle ended, etc)
    fn step(&mut self, cmd: &str, player: &Player) -> bool;

    /// used to statically check if the state is complete
    fn is_done(&self) -> bool;
}

pub struct ExploreState {
    /// which zone in the level the player is in.
    pub loc: (u8, u8),
}

impl ExploreState {
    fn new() -> Self {
        Self { loc: (128, 128) }
    }
}

impl GameState for ExploreState {
    fn is_done(&self) -> bool {
        false
    }

    #[allow(unused_variables)]
    fn step(&mut self, cmd: &str, player: &Player) -> bool {
        // TODO: parse cmd and do the thing the player instructed

        self.is_done()
    }
}

pub struct StateStack {
    stack: Vec<Rc<RefCell<dyn GameState>>>,
}

impl StateStack {
    pub fn new() -> Self {
        let mut stack: Vec<Rc<RefCell<dyn GameState>>> = Vec::with_capacity(3);
        stack.push(Rc::from(RefCell::from(ExploreState::new())));

        Self { stack }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn try_pop(&mut self) -> anyhow::Result<()> {
        match self.stack.last() {
            Some(state) if state.borrow_mut().is_done() => {
                self.pop();
                Ok(())
            }
            _ => bail!("state still running!"),
        }
    }

    pub fn step(&mut self, cmd: &str, player: &Player) -> bool {
        match self.stack.last() {
            Some(state) => state.borrow_mut().step(cmd, player),
            None => false,
        }
    }
}

pub struct Game {
    pub world: World,
    // level: Level,
    pub player: Player,
    pub state: StateStack,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            player: Player::new(None),
            state: StateStack::new(), // Box::new(ExploreState::new()),
        }
    }

    pub fn step(&mut self, cmd: &str) {
        if self.state.step(cmd, &self.player) {
            self.state.pop();
        }
    }
}
