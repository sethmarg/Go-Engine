//! A custom agent that plays the game of Go.
//!
//! This program follows the Elm Architecture, with [`board`] as the model.

//#![deny(missing_docs)]
#![warn(missing_docs)]

pub mod board;
pub mod update;
pub mod view;

mod groups;

use board::Color;
use update::{update, Message, Position};

/// Starting point of the program. Command line arguments detail functionality.
///
/// `-- gtp` starts a Go Text Protocol listener on `std::in`, outputs on `std::out` and `std::err`.
///
/// `-- api` starts a Go Text Protocol listener for HTTP requests on port 80.
pub fn main() {
    use engine::start_go_agent;
    use std::env;

    let arguments: Vec<String> = env::args().collect();

    if arguments.len() > 1 && arguments[1].eq_ignore_ascii_case("debug") {
        debug();
    } else {
        start_go_agent(arguments);
    }
}

#[doc(hidden)]
/// Debug method for testing work in progress code.
fn debug() {
    let mut board = board::Board::new();
    
    let move_list = vec![
        (Color::Black, Position {row: 4, col: 4}),
        (Color::White, Position {row: 3, col: 4}),
        (Color::Black, Position {row: 3, col: 5}),
        (Color::White, Position {row: 2, col: 5}),
        (Color::Black, Position {row: 4, col: 6}),
        (Color::White, Position {row: 3, col: 6}),
        (Color::Black, Position {row: 5, col: 5}),
        (Color::White, Position {row: 4, col: 5}),
        (Color::Black, Position {row: 3, col: 5}),
    ];
    
    println!("{board}\n");
    
    for mov in move_list {
        update(&mut board, Message::Play(mov.0, mov.1)).expect("Something went wrong");
        println!("{board}\n")
    }
}
