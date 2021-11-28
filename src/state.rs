use std::hash::Hash;

#[derive(Clone, Copy, Debug)]
pub enum Strategy {
    Min,
    Max,
    MinAvg,
    MaxAvg,
    Avg,
}

pub trait GameState<T>: Eq + Hash {
    fn start() -> Self;
    fn done(&self) -> Option<T>;
    fn heuristic(&self) -> T;
    fn strategy(&self) -> Strategy;
    fn actions(&self) -> Vec<Self>
    where
        Self: Sized;
}
