use crate::minimax::minimax;
use crate::oxo::OxoState;
use crate::state::{GameState, MinMaxVisitor};
use crate::visitors::{GameplanVisitor, OutcomeVisitor};
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashMap;

mod minimax;
mod oxo;
mod state;
mod visitors;

fn print_oxo_outcome_info() {
    let state = OxoState::start();
    let mut states = HashMap::new();

    let mut visitor = OutcomeVisitor::default();
    let value = minimax(&state, &mut states, 100, &mut visitor);

    let outcome = match value.partial_cmp(&0.0) {
        None => "Error",
        Some(Ordering::Equal) => "Draw",
        Some(Ordering::Less) => "Black wins",
        Some(Ordering::Greater) => "White wins",
    };

    println!("visited game states: {}", states.len());
    println!();
    println!("expected outcome if optimal play: {}", outcome);
    println!();
    println!("possible end states:");
    println!("\t Draws: {}", visitor.draws);
    println!("\t White Wins: {}", visitor.wins);
    println!("\t Black Wins: {}", visitor.losses);
}

fn random_oxo_state(depth: u64) -> OxoState {
    let mut current = OxoState::start();
    for _ in 0..depth {
        match current.actions().choose(&mut rand::thread_rng()) {
            Some(x) => current = x.clone(),
            None => return current,
        }
    }
    current
}

fn print_oxo_game_plan() {
    let state = OxoState::start();
    let mut states = HashMap::new();

    let mut visitor = GameplanVisitor::default();
    let _ = minimax(&state, &mut states, 100, &mut visitor);

    let mut round = 1;
    let mut current = state.clone();
    while let Some((new, _)) = visitor.game_plan.get(&current) {
        current = new.clone();
        println!("optimal action for round {}", round);
        println!("{}", current);

        round += 1;
    }
}

fn print_actions_for_random_state() {
    let random = random_oxo_state(5);

    println!("start state:");
    println!("{}", random);
    println!("next states:");

    for child in random.actions() {
        println!("{}", child)
    }
}

fn main() {
    print_actions_for_random_state()
}
