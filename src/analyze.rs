use crate::state::{GameState, Strategy};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::{Add, Div};
use std::process::Output;

// fn game_tree(start: State) {
//     let mut queue = VecDeque::new();
//     let mut visited = HashSet::new();
//     queue.push_back(start);
//
//     while let Some(current) = queue.pop_front() {
//         if !visited.insert(current) {
//             continue;
//         }
//
//         if let Some(_) = current.done() {
//             println!("x won")
//         } else {
//             queue.extend(current.actions())
//         }
//     }
// }

fn minavg<T: Default + Clone + Add<Output = T> + Div<f64, Output = T> + PartialOrd>(
    iter: impl Iterator<Item = T>,
) -> T {
    let mut accum = T::default();
    let mut lowest = T::default();
    let mut len = 0;

    for value in iter {
        if value < lowest {
            lowest = value.clone()
        }
        accum = accum + value;
        len += 1;
    }

    if lowest < T::default() {
        lowest
    } else {
        accum / len as f64
    }
}

pub fn minimax<S, M, T, P>(
    node: S,
    visited: &mut HashMap<S, T>,
    depth: u64,
    strategy: Strategy,
    modify: &M,
    meta: &mut P,
) -> T
where
    S: GameState<T>,
    M: Fn(&S, T, &mut P),
    T: Clone + Default + Ord + Add<Output = T> + Div<f64, Output = T> + std::iter::Sum,
    P: Default,
{
    // Check the visited nodes and return the value if already in game tree
    if let Some(value) = visited.get(&node) {
        return value.clone();
    }

    // If the node is in a finished state, return it's done value
    if let Some(value) = node.done() {
        visited.insert(node, value.clone());
        return value;
    }

    // Stop if too deep and return the heuristic
    if depth > 100 {
        // We don't insert into the visited map
        return node.heuristic();
    }

    // Calculate the actions
    let actions = node.actions();
    let length = actions.len();

    // No more valid action and no victory so it's a draw
    if actions.is_empty() {
        visited.insert(node, T::default());
        return T::default();
    }

    // Do the minimax recursive step
    let children = actions
        .into_iter()
        .map(|child| minimax(child, visited, depth + 1, strategy, modify, meta));

    let result = match node.strategy() {
        Strategy::Min => children.min().unwrap(),
        Strategy::Max => children.max().unwrap(),
        Strategy::MinAvg => minavg(children),
        Strategy::MaxAvg => todo!(),
        Strategy::Avg => children.sum::<T>() / length as f64,
    };

    visited.insert(node, result.clone());
    result
}
