mod board;
mod gtp;
mod tests;
mod engine;

use board::*;
use engine::*;
use wasm_bindgen::prelude::*;
/****************************************************\
|****************        MAIN        ****************|
\****************************************************/

fn debug() {
    use ColumnIdentifier::*;
    let mut b: Board = Board::new(BoardSize::NINETEEN);
    
    println!("{:?}", generate_move(&b, Color::BLACK, 30));
}

#[wasm_bindgen]
pub fn start_go_agent(args: Vec<String>) {
    use gtp::GTP;
    if args.len() < 2 {
        panic!("Run mode not given");
    }

    if args[1].eq_ignore_ascii_case("debug") {
        debug();
    } else if args[1].eq_ignore_ascii_case("gtp") {
        let gtp: GTP = GTP::new();
        gtp.start().expect("Something went wrong during GTP loop");
    } else {
        panic!(
            "Invalid run mode given: {}\n Run mode must be debug or gtp",
            args[1]
        );
    }
}
