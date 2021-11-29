use crate::state::{GameState, MinMaxVisitor, Strategy};
use std::collections::HashMap;

fn minavg(iter: impl Iterator<Item = f64>, len: usize) -> f64 {
    let mut accum: f64 = 0.;
    let mut lowest: f64 = 0.;

    for value in iter {
        if value > 0.0 {
            println!("feest")
        }
        lowest = lowest.min(value);
        accum = accum + value;
    }
    match lowest > 0.0 {
        true => accum / (len as f64),
        false => lowest,
    }
}

pub fn argmax(iter: impl Iterator<Item = f64>) -> (usize, f64) {
    iter.enumerate()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
}

pub fn argmin(iter: impl Iterator<Item = f64>) -> (usize, f64) {
    iter.enumerate()
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
}
//
// pub fn apply_strategy(
//     children: impl Iterator<Item = f64>,
//     strategy: Strategy,
//     buffer: &mut Vec<f64>,
// ) -> () {
//     // Clear and refill the vector
//     buffer.clear();
//     buffer.extend(children);
// }

pub fn minimax<S, V>(node: &S, visited: &mut HashMap<S, f64>, depth: u64, visitor: &mut V) -> f64
where
    S: GameState,
    V: MinMaxVisitor<S>,
{
    // Check the visited nodes and return the value if already in game tree
    if let Some(value) = visited.get(&node) {
        return *value;
    }

    // If the node is in a finished state, return it's done value
    if let Some(value) = node.done() {
        visitor.finished(node, value);
        visited.insert(node.clone(), value);
        return value;
    }

    // Stop if too deep and return the heuristic
    if depth == 0 {
        // We don't insert into the visited map
        return node.heuristic();
    }

    // Calculate the actions
    let actions = node.actions();
    let length = actions.len();

    // No more valid action and no victory so it's a draw
    if actions.is_empty() {
        visitor.finished(node, 0.);
        visited.insert(node.clone(), 0.);
        return 0.;
    }

    // Do the minimax recursive step
    let children = actions
        .iter()
        .map(|child| minimax(child, visited, depth + 1, visitor));

    let (next, value) = match node.strategy() {
        Strategy::Min => {
            let (index, value) = argmin(children);
            (actions[index].clone(), value)
        }
        Strategy::Max => {
            let (index, value) = argmax(children);
            (actions[index].clone(), value)
        }
        Strategy::MinAvg => todo!(),
        Strategy::MaxAvg => todo!(),
        Strategy::Avg => todo!(),
    };

    visitor.visit(node, &next, value);
    visited.insert(node.clone(), value);
    value
}
