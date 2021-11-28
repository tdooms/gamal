use crate::analyze::minimax;
use crate::oxo::OxoState;
use crate::state::GameState;
use std::collections::{HashMap, HashSet, VecDeque};

mod analyze;
mod oxo;
mod state;

fn minimax_info() {
    let state = OxoState::start();

    let mut states = HashMap::new();

    let modify = |_, _, _| ();
    let value = minimax(state, &mut states, 0, state.strategy(), &modify, &mut ());

    let outcome = match value {
        x if x > 0 => "White wins",
        0 => "Draw",
        x if x < 0 => "Black wins",
        _ => "unknown",
    };

    println!("visited game states: {}", states.len());
    println!();
    println!("expected outcome if optimal play: {}", outcome);
    println!();
    println!("possible end states:");
    // println!("\t Draws: {}", meta.0);
    // println!("\t White Wins: {}", meta.1);
    // println!("\t Black Wins: {}", meta.2);
}

fn main() {
    minimax_info()
}
