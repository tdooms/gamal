use std::hash::Hash;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Strategy {
    Min,
    Max,
    MinAvg,
    MaxAvg,
    Avg,
}

pub trait GameState: Eq + Hash + Clone {
    fn start() -> Self;
    fn done(&self) -> Option<f64>;
    fn heuristic(&self) -> f64;
    fn strategy(&self) -> Strategy;
    fn actions(&self) -> Vec<Self>
    where
        Self: Sized;
}

pub trait MinMaxVisitor<S: GameState> {
    fn finished(&mut self, node: &S, value: f64);
    fn visit(&mut self, node: &S, next: &S, value: f64);
}
