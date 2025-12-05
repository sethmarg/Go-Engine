//! Computations regarding groups of stones.

use crate::board::{BoardSize, Color, State};
use std::collections::{HashSet, VecDeque};

/// A group of [`Color`] stones on a Go Board.
pub(crate) struct Group {
    /// The list of position indexes of stones in the group.
    pub(crate) stones: Vec<usize>,
    /// The list of position indexes of liberties in the group.
    pub(crate) liberties: Vec<usize>,
    /// The color of stones in the group.
    pub(crate) color: Color,
}

/// Calculates the neighboring position indexes of the given index if the given index is valid.
pub(crate) fn neighbors(index: usize, board: &Vec<State>, size: &BoardSize) -> Vec<usize> {
    match board[index] {
        State::Offboard => vec![],
        _ => vec![
            index + 1,
            index - 1,
            index + size.to_u16() as usize + 2,
            index - size.to_u16() as usize - 2,
        ],
    }
}

/// Finds the group of stones on the board of [`Color`] connected to the stone at the given index.
pub(crate) fn find_group(
    start_index: usize,
    color: &Color,
    board: &Vec<State>,
    size: &BoardSize,
) -> Group {
    let mut seen_set: HashSet<usize> = HashSet::new();
    let mut worklist: VecDeque<usize> = VecDeque::new();
    let mut group: Vec<usize> = vec![];
    let mut liberties: Vec<usize> = vec![];

    worklist.push_back(start_index);
    seen_set.insert(start_index);

    while !worklist.is_empty() {
        let cur_index = worklist.pop_front().unwrap();

        match board[cur_index] {
            State::Empty => liberties.push(cur_index),
            State::Occupied(c) => {
                if c == *color {
                    group.push(cur_index);
                    worklist = worklist
                        .into_iter()
                        .chain(
                            neighbors(cur_index, board, size)
                                .into_iter()
                                .filter(|n| !seen_set.contains(n)),
                        )
                        .collect();

                    seen_set = seen_set
                        .into_iter()
                        .chain(neighbors(cur_index, board, size).into_iter())
                        .collect();
                }
            }
            State::Offboard => {}
        }
    }

    Group {
        stones: group,
        liberties,
        color: *color,
    }
}
