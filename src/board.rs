//! The game board of Go.
//!
//! Supports 9x9, 13x13, and 19x19 board sizes.

use std::fmt;
use std::fmt::Formatter;

/// The colors of stones on a Go Board.
#[derive(Copy, Clone, Debug)]
enum Color {
    Black,
    White,
}

/// The states of intersections on a Go Board
#[derive(Copy, Clone, Debug)]
enum State {
    /// An empty intersection
    Empty,
    /// An intersection occupied by a stone of a given ['Color']
    Occupied(Color),
    #[doc(hidden)]
    /// A sentinel value that borders the board for ease of computation.
    Offboard,
}

/// The supported sizes of Go Boards.
///
/// Currently, supports the standard 9x9, 13x13, and 19x19 sizes.
pub enum BoardSize {
    /// Board size of 9x9.
    Nine,
    /// Board size of 13x13.
    Thirteen,
    /// Board size of 19x19.
    Nineteen,
}

/// Represents a Go Board
pub struct Board {
    size: BoardSize,
    board: Vec<State>,
    ko: Option<usize>,
    black_captures: u16,
    white_captures: u16,
}

impl BoardSize {
    /// Converts a [`BoardSize`] to its numeric representation.
    fn to_u16(&self) -> u16 {
        match self {
            BoardSize::Nine => 9,
            BoardSize::Thirteen => 13,
            BoardSize::Nineteen => 19,
        }
    }
}

impl Board {
    /// Constructs a new empty [`Board`]. Default size is [`19x19`](BoardSize::Nineteen)
    pub fn new() -> Self {
        Board {
            size: BoardSize::Nineteen,
            board: init_board(BoardSize::Nineteen),
            ko: None,
            black_captures: 0,
            white_captures: 0,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let line_render = self
            .board
            .iter()
            .fold(String::new(), |acc, state| match state {
                State::Empty => format!("{acc}."),
                State::Occupied(color) => format!("{acc}{color}"),
                State::Offboard => acc,
            });
        
        let render = line_render
            .chars()
            .enumerate()
            .flat_map(|(i, c)| {
                if i != 0 && i % self.size.to_u16() as usize == 0 {
                    Some('\n')
                } else {
                    None
                }
                .into_iter()
                .chain(std::iter::once(c))
            })
            .map(|c| { format!(" {c}")})
            .collect::<String>();
        
        write!(f, "{render}")
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Color::Black => write!(f, "X"),
            Color::White => write!(f, "0"),
        }
    }
}

#[doc(hidden)]
/// Initializes an empty board vector of the given [`BoardSize`].
fn init_board(size: BoardSize) -> Vec<State> {
    let mut board: Vec<State> = vec![];
    let row_len = size.to_u16() + 2;

    for i in 0..row_len * row_len {
        if i / row_len == 0 || i / row_len == row_len - 1 {
            board.push(State::Offboard)
        } else if i % row_len == row_len - 1 || i % row_len == 0 {
            board.push(State::Offboard);
        } else {
            board.push(State::Empty);
        }
    }

    board
}
