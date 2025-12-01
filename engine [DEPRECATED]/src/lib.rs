mod board;
mod gtp;
mod tests;
mod engine;
mod api;

use board::*;
use engine::*;
use gtp::*;
use api::*;
/****************************************************\
|****************        MAIN        ****************|
\****************************************************/

// Debug commands
fn debug() {
    use ColumnIdentifier::*;
    let mut b: Board = Board::new(BoardSize::NINETEEN);
    
    println!("{:?}", generate_move(&b, Color::BLACK, 30));
}

// Main library function, controls the runmode of the program
pub fn start_go_agent(args: Vec<String>) {
    use gtp::GTP;
    if args.len() < 2 {
        panic!("Run mode not given");
    }

    if args[1].eq_ignore_ascii_case("debug") {
        debug();
    } else if args[1].eq_ignore_ascii_case("gtp") {
        let gtp: GTP = GTP::new();
        gtp.start_listener().expect("Something went wrong during GTP loop");
    } else if args[1].eq_ignore_ascii_case("api") {
        start_api();
    } else {
        panic!(
            "Invalid run mode given: {}\n Run mode must be debug or gtp",
            args[1]
        );
    }
}
