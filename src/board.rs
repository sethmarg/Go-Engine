//! The game board of Go.
//!
//! Supports 9x9, 13x13, and 19x19 board sizes.

use std::fmt;
use std::fmt::Formatter;
use crate::groups;

/// The colors of stones on a Go Board.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Color {
    /// Black stones.
    Black,
    /// White stones.
    White,
}

/// The states of intersections on a Go Board
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum State {
    /// An empty intersection
    Empty,
    /// An intersection occupied by a stone of a given [`Color`]
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
    pub(crate) size: BoardSize,
    pub(crate) board: Vec<State>,
    pub(crate) ko: Option<usize>,
    pub(crate) black_captures: u16,
    pub(crate) white_captures: u16,
    pub(crate) player_turn: Color,
    pub(crate) move_number: u16,
}

impl BoardSize {
    /// Converts a [`BoardSize`] to its numeric representation.
    pub fn to_u16(&self) -> u16 {
        match self {
            BoardSize::Nine => 9,
            BoardSize::Thirteen => 13,
            BoardSize::Nineteen => 19,
        }
    }
}

impl Color {
    /// Returns the opposite [`Color`] of the current.
    pub fn opposite_color(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl Board {
    /// Constructs a new empty [`Board`]. Default size is [`19x19`](BoardSize::Nineteen)
    pub fn new() -> Self {
        Board {
            size: BoardSize::Nineteen,
            board: init_board(&BoardSize::Nineteen),
            ko: None,
            black_captures: 0,
            white_captures: 0,
            player_turn: Color::Black,
            move_number: 0,
        }
    }

    pub(crate) fn attempt_captures(&mut self, played_index: usize, played_color: &Color) {
        let mut potential_kos: Vec<usize> = vec![]; // todo: probably better way to deal with ko
        
        for start_index in groups::neighbors(played_index, &self.board, &self.size) {
            let group = groups::find_group(
                start_index,
                &played_color.opposite_color(),
                &self.board,
                &self.size,
            );

            if group.liberties.len() == 0 {
                if self.capture_causes_ko(&group) {
                    potential_kos.push(group.stones[0]); // guaranteed to be a group of size 1
                }
                
                group.stones.iter().for_each(|index| self.board[*index] = State::Empty);
                match played_color {
                    Color::White => self.white_captures += group.stones.len() as u16,
                    Color::Black => self.black_captures += group.stones.len() as u16,
                }
            }
        }

        self.ko = if potential_kos.len() == 1 {
            Some(potential_kos[0])
        } else {
            None
        }
    }

    fn capture_causes_ko(&mut self, captured_group: &groups::Group) -> bool {
        if captured_group.stones.len() == 1 {
            groups::neighbors(captured_group.stones[0], &self.board, &self.size)
                .iter()
                .map(|index| self.board[*index])
                .all(|state| state == State::Occupied(captured_group.color.opposite_color()) || state == State::Offboard)
        } else {
            false
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
            .map(|c| format!(" {c}"))
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
pub(crate) fn init_board(size: &BoardSize) -> Vec<State> {
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
