mod board;
mod gtp;
mod tests;

use board::*;
/****************************************************\
|****************        MAIN        ****************|
\****************************************************/

fn debug() {
    use ColumnIdentifier::*;
    let mut b: Board = Board::new(BoardSize::NINETEEN);
    b.play(Move::MOVE(Intersection::new(B, 2), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(B, 3), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(C, 2), Color::WHITE));
    b.play(Move::MOVE(Intersection::new(C, 3), Color::WHITE));
    // b.position[Intersection { column: B, row: 2 }.to_position_index(&b.size) as usize] =
    //     State::OCCUPIED(Color::WHITE);
    // b.position[Intersection { column: B, row: 3 }.to_position_index(&b.size) as usize] =
    //     State::OCCUPIED(Color::WHITE);
    // b.position[Intersection { column: C, row: 2 }.to_position_index(&b.size) as usize] =
    //     State::OCCUPIED(Color::WHITE);
    // b.position[Intersection { column: C, row: 3 }.to_position_index(&b.size) as usize] =
    //     State::OCCUPIED(Color::WHITE);
    print!("{}", b.render());
    let (group, liberties) = b.count(
        Intersection::new(B, 2).to_position_index(&b.size).unwrap() as usize,
        Color::WHITE,
    );
    println!("Group: {:#?}", group);
    println!("Liberties: {:#?}", liberties);
}

fn main() {
    use gtp::GTP;
    use std::env;
    let args: Vec<String> = env::args().collect();
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
