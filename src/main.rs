//! A custom agent that plays the game of Go.

#![warn(missing_docs)]

pub mod board;

/// Starting point of the program. Command line arguments detail functionality.
/// 
/// `-- gtp` starts a Go Text Protocol listener on `std::in`, outputs on `std::out` and `std::err`.
/// 
/// `-- api` starts a Go Text Protocol listener for HTTP requests on port 80.
pub fn main() {
    use std::env;
    use engine::start_go_agent;
    
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
    println!("{}", board::Board::new());
}