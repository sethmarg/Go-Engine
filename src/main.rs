mod tests;
mod board;
use board::*;

/****************************************************\
|****************        MAIN        ****************|
\****************************************************/

fn main() {
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
        Intersection::new(B, 2)
            .to_position_index(&b.size)
            .unwrap() as usize,
        Color::WHITE,
    );
    println!("Group: {:#?}", group);
    println!("Liberties: {:#?}", liberties);
}