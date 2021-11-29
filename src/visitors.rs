use crate::{GameState, MinMaxVisitor, OxoState};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Default)]
pub struct OutcomeVisitor {
    pub draws: u64,
    pub wins: u64,
    pub losses: u64,

    pub inner: u64,
    pub leaf: u64,
}

impl MinMaxVisitor<OxoState> for OutcomeVisitor {
    fn finished(&mut self, _node: &OxoState, value: f64) {
        match value.partial_cmp(&0.0) {
            None => {}
            Some(Ordering::Equal) => self.draws += 1,
            Some(Ordering::Less) => self.losses += 1,
            Some(Ordering::Greater) => self.wins += 1,
        }
        self.leaf += 1
    }

    fn visit(&mut self, _node: &OxoState, _next: &OxoState, _value: f64) {
        self.inner += 1
    }
}

pub struct GameplanVisitor<S: GameState> {
    pub game_plan: HashMap<S, (S, f64)>,
}

impl<S: GameState> Default for GameplanVisitor<S> {
    fn default() -> Self {
        Self {
            game_plan: Default::default(),
        }
    }
}

impl MinMaxVisitor<OxoState> for GameplanVisitor<OxoState> {
    fn finished(&mut self, node: &OxoState, value: f64) {}

    fn visit(&mut self, node: &OxoState, next: &OxoState, value: f64) {
        self.game_plan.insert(node.clone(), (next.clone(), value));
    }
}
