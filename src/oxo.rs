use crate::state::{GameState, Strategy};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::thread::current;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Hash, Eq)]
enum Piece {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
enum Player {
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub struct OxoState {
    squares: [Option<(Piece, Player)>; 9],
    turn: Player,

    pieces: [[u8; 3]; 2],
}

impl Display for OxoState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let piece_fmt = |opt| match opt {
            None => "__",
            Some((Piece::Small, Player::White)) => "sw",
            Some((Piece::Medium, Player::White)) => "mw",
            Some((Piece::Large, Player::White)) => "lw",
            Some((Piece::Small, Player::Black)) => "sb",
            Some((Piece::Medium, Player::Black)) => "mb",
            Some((Piece::Large, Player::Black)) => "lb",
        };

        writeln!(
            f,
            "Small: {:?}, Medium: {:?}, Large: {:?}",
            self.pieces[0][0], self.pieces[0][1], self.pieces[0][2]
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "\t\t  {} {} {}",
            piece_fmt(self.squares[0]),
            piece_fmt(self.squares[1]),
            piece_fmt(self.squares[2])
        )?;
        writeln!(
            f,
            "\t\t  {} {} {}",
            piece_fmt(self.squares[3]),
            piece_fmt(self.squares[4]),
            piece_fmt(self.squares[5])
        )?;
        writeln!(
            f,
            "\t\t  {} {} {}",
            piece_fmt(self.squares[6]),
            piece_fmt(self.squares[7]),
            piece_fmt(self.squares[8])
        )?;
        writeln!(f)?;
        writeln!(
            f,
            "Small: {:?}, Medium: {:?}, Large: {:?}",
            self.pieces[1][0], self.pieces[1][1], self.pieces[1][2]
        )
    }
}

impl OxoState {
    pub fn modify(&self, idx: usize, piece: Piece) -> Option<Self> {
        if self.pieces[self.turn as usize][piece as usize] == 0 {
            return None;
        }

        match self.squares[idx] {
            None => (),
            Some((old, player)) if player != self.turn && piece <= old => return None,
            Some(_) => return None,
        }

        let mut copy = *self;
        copy.pieces[self.turn as usize][piece as usize] -= 1;
        copy.squares[idx] = Some((piece, self.turn));

        copy.turn = match self.turn {
            Player::White => Player::Black,
            Player::Black => Player::White,
        };

        Some(copy)
    }
}

impl GameState<i64> for OxoState {
    fn start() -> Self {
        Self {
            squares: [None; 9],
            turn: Player::White,
            pieces: [[3, 2, 2]; 2],
        }
    }

    fn done(&self) -> Option<i64> {
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        let all_same = |line: [], player| {
            line.iter()
                .all(|idx| self.squares[*idx].map(|x| x.1) == Some(player))
        };

        for line in LINES {
            if all_same(line, Player::White) {
                return Some(1);
            } else if all_same(line, Player::Black) {
                return Some(-1);
            }
        }
        None
    }

    fn heuristic(&self) -> i64 {
        // TODO: something smart
        0
    }

    fn strategy(&self) -> Strategy {
        match self.turn {
            Player::White => Strategy::Max,
            Player::Black => Strategy::MinAvg,
        }
    }

    fn actions(&self) -> Vec<OxoState> {
        const PIECES: [Piece; 3] = [Piece::Small, Piece::Medium, Piece::Large];
        (0..8)
            .map(move |idx| PIECES.into_iter().map(move |piece| self.modify(idx, piece)))
            .flatten()
            .filter_map(|x| x)
            .collect()
    }
}
