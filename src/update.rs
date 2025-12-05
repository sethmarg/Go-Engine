//! Logic that updates the Go board depending on the received command.

use crate::board::{init_board, Board, BoardSize, Color, State};
use crate::groups;

/// Requests that can be sent to [`update`] which alter the state of the program.
pub enum Message {
    /// An empty message.
    None,
    /// Plays a stone of [`Color`] at the given [`Position`] if it follows the rules of Go.
    Play(Color, Position),
    /// Places a stone of [`Color`] at the given [`Position`] without considering turn or capture logic.
    PlaceStone(Color, Position),
    /// Passes the current player's turn.
    Pass,
    /// Indicates the current player wishes to resign.
    Resign,
    /// Clears the current board state.
    Clear,
    /// Sets the size of the board to [`BoardSize`].
    SetSize(BoardSize),
}

/// Updates the given [`Board`] according to the [`Message`] received.
///
/// Outputs the next requested [`Message`] on a successful operation, or a [`String`] detailing
/// why the operation failed.
pub fn update(board: &mut Board, msg: Message) -> Result<Message, String> {
    match msg {
        Message::None => Ok(Message::None),
        Message::Play(color, pos) => {
            if board.player_turn == color {
                update(board, Message::PlaceStone(color, pos)).and_then(|_| {
                    let index = pos.to_board_index(&board.size).unwrap();
                    if board.ko.is_none_or(|ko| ko != index) {
                        board.attempt_captures(index, &color);
                        if groups::find_group(index, &color, &board.board, &board.size)
                            .liberties
                            .len()
                            != 0
                        {
                            board.player_turn = board.player_turn.opposite_color();
                            board.ko = None;
                            board.move_number += 1;
                            Ok(Message::None)
                        } else {
                            board.board[index] = State::Empty;
                            Err("Placing a stone at this intersection is suicidal".to_string())
                        }
                    } else {
                        Err(
                            "Placing a stone at this intersection violates the rule of ko"
                                .to_string(),
                        )
                    }
                })
            } else {
                Err("Playing this move violates the turn order".to_string())
            }
        }
        Message::PlaceStone(color, pos) => {
            if let Some(index) = pos.to_board_index(&board.size) {
                match board.board[index] {
                    State::Empty => {
                        board.board[index] = State::Occupied(color);
                        Ok(Message::None)
                    }
                    State::Occupied(_) => {
                        Err("Cannot place stone at occupied intersection".to_string())
                    }
                    State::Offboard => Err("Should be impossible how".to_string()),
                }
            } else {
                Err("Intersection is out of bounds for current boardsize".to_string())
            }
        }
        Message::Pass => {
            board.player_turn = board.player_turn.opposite_color();
            board.move_number += 1;
            board.ko = None;
            Ok(Message::None)
        }
        Message::Resign => todo!(),
        Message::Clear => {
            board.board = init_board(&board.size);
            board.ko = None;
            board.black_captures = 0;
            board.white_captures = 0;
            Ok(Message::None)
        }
        Message::SetSize(size) => {
            board.size = size;
            update(board, Message::Clear)
        }
    }
}

/// An indexed position on the Go Board.
#[derive(Copy, Clone)]
pub struct Position {
    pub(crate) row: u16,
    pub(crate) col: u16,
}

impl Position {
    /// Computes the position index of this [`Position`] on a board of size [`BoardSize`], if valid.
    pub(crate) fn to_board_index(&self, boardsize: &BoardSize) -> Option<usize> {
        let numeric_boardsize = match boardsize {
            BoardSize::Nine => 9u16,
            BoardSize::Thirteen => 13u16,
            BoardSize::Nineteen => 19u16,
        };

        if self.col + self.row * numeric_boardsize >= numeric_boardsize.pow(2) {
            None
        } else {
            let vector_row_length = numeric_boardsize + 2;
            let row_index = (vector_row_length - self.row - 2) * vector_row_length;
            Some(usize::from(self.col + row_index + 1))
            // Some(usize::from(
            //     vector_row_length + (self.row * numeric_boardsize) + self.col + 1,
            // ))
        }
    }
}
