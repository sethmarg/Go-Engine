#[cfg(test)]
use super::*;
/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

#[test]
fn test_board_deepcopy() {
    let mut board = Board::new(BoardSize::NINETEEN);
    let board_copy = board.deepcopy();
    assert_eq!(board, board_copy); // copies of each other

    board.play(Move::MOVE(
        Intersection::new(ColumnIdentifier::A, 1),
        Color::WHITE,
    ));

    assert_ne!(board, board_copy); // board has updated, board_copy has not
}

/****************************************************\
|****************       HELPER       ****************|
\****************************************************/

#[test]
fn test_add_usize() {
    assert_eq!(add_to_usize(10, 5), Some(15)); // ensure adding works
    assert_eq!(add_to_usize(7, -5), Some(2)); // ensure subtraction works
    assert_eq!(add_to_usize(2, -3), None); // ensure underflow returns None
    assert_eq!(add_to_usize(usize::MAX, 1), None); // ensures overflow returns None
}

#[test]
fn test_board_size_from_u16() {
    assert_eq!(BoardSize::from_u16(9), Some(BoardSize::NINE));
    assert_eq!(BoardSize::from_u16(13), Some(BoardSize::THIRTEEN));
    assert_eq!(BoardSize::from_u16(19), Some(BoardSize::NINETEEN));
    assert_eq!(BoardSize::from_u16(0), None); // random number check
    assert_eq!(BoardSize::from_u16(35), None); // second random number check
}

#[test]
fn test_board_size_to_u16() {
    assert_eq!(BoardSize::NINE.to_u16(), 9);
    assert_eq!(BoardSize::THIRTEEN.to_u16(), 13);
    assert_eq!(BoardSize::NINETEEN.to_u16(), 19);
}

#[test]
fn test_column_identifier_from_u16() {
    use ColumnIdentifier::*;
    assert_eq!(ColumnIdentifier::from_u16(0), Some(A));
    assert_eq!(ColumnIdentifier::from_u16(1), Some(B));
    assert_eq!(ColumnIdentifier::from_u16(2), Some(C));
    assert_eq!(ColumnIdentifier::from_u16(3), Some(D));
    assert_eq!(ColumnIdentifier::from_u16(4), Some(E));
    assert_eq!(ColumnIdentifier::from_u16(5), Some(F));
    assert_eq!(ColumnIdentifier::from_u16(6), Some(G));
    assert_eq!(ColumnIdentifier::from_u16(7), Some(H));
    assert_eq!(ColumnIdentifier::from_u16(8), Some(J));
    assert_eq!(ColumnIdentifier::from_u16(9), Some(K));
    assert_eq!(ColumnIdentifier::from_u16(10), Some(L));
    assert_eq!(ColumnIdentifier::from_u16(11), Some(M));
    assert_eq!(ColumnIdentifier::from_u16(12), Some(N));
    assert_eq!(ColumnIdentifier::from_u16(13), Some(O));
    assert_eq!(ColumnIdentifier::from_u16(14), Some(P));
    assert_eq!(ColumnIdentifier::from_u16(15), Some(Q));
    assert_eq!(ColumnIdentifier::from_u16(16), Some(R));
    assert_eq!(ColumnIdentifier::from_u16(17), Some(S));
    assert_eq!(ColumnIdentifier::from_u16(18), Some(T));
    assert_eq!(ColumnIdentifier::from_u16(19), None); // random num check
}

#[test]
fn test_column_identifier_to_u16() {
    use ColumnIdentifier::*;
    assert_eq!(A.to_u16(), 0);
    assert_eq!(B.to_u16(), 1);
    assert_eq!(C.to_u16(), 2);
    assert_eq!(D.to_u16(), 3);
    assert_eq!(E.to_u16(), 4);
    assert_eq!(F.to_u16(), 5);
    assert_eq!(G.to_u16(), 6);
    assert_eq!(H.to_u16(), 7);
    assert_eq!(J.to_u16(), 8);
    assert_eq!(K.to_u16(), 9);
    assert_eq!(L.to_u16(), 10);
    assert_eq!(M.to_u16(), 11);
    assert_eq!(N.to_u16(), 12);
    assert_eq!(O.to_u16(), 13);
    assert_eq!(P.to_u16(), 14);
    assert_eq!(Q.to_u16(), 15);
    assert_eq!(R.to_u16(), 16);
    assert_eq!(S.to_u16(), 17);
    assert_eq!(T.to_u16(), 18);
}

#[test]
fn test_column_identifier_to_string() {
    use ColumnIdentifier::*;
    assert_eq!(A.to_string(), "A");
    assert_eq!(B.to_string(), "B");
    assert_eq!(C.to_string(), "C");
    assert_eq!(D.to_string(), "D");
    assert_eq!(E.to_string(), "E");
    assert_eq!(F.to_string(), "F");
    assert_eq!(G.to_string(), "G");
    assert_eq!(H.to_string(), "H");
    assert_eq!(J.to_string(), "J");
    assert_eq!(K.to_string(), "K");
    assert_eq!(L.to_string(), "L");
    assert_eq!(M.to_string(), "M");
    assert_eq!(N.to_string(), "N");
    assert_eq!(O.to_string(), "O");
    assert_eq!(P.to_string(), "P");
    assert_eq!(Q.to_string(), "Q");
    assert_eq!(R.to_string(), "R");
    assert_eq!(S.to_string(), "S");
    assert_eq!(T.to_string(), "T");
}

#[test]
fn test_intersection_to_position_index() {
    use ColumnIdentifier::*;
    let intsc1 = Intersection::new(A, 1); // works on every board
    let intsc2 = Intersection::new(K, 3); // does not work on 9x9, too high of column
    let intsc3 = Intersection::new(J, 10); // does not work on 9x9, too high of row
    let intsc4 = Intersection::new(M, 12); // does not work on 9x9, both dimensions
    let intsc5 = Intersection::new(Q, 5); // only works on 19x19, too high of column
    let intsc6 = Intersection::new(B, 15); // only works on 19x19, too high of row
    let intsc7 = Intersection::new(S, 16); // only works on 19x19, both dimensions
    let intsc8 = Intersection::new(A, 20); // never works
    let intsc9 = Intersection::new(O, 0); // too low of row, never works

    assert_eq!(intsc1.to_position_index(&BoardSize::NINE), Some(100));
    assert_eq!(intsc1.to_position_index(&BoardSize::THIRTEEN), Some(196));
    assert_eq!(intsc1.to_position_index(&BoardSize::NINETEEN), Some(400));

    assert_eq!(intsc2.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc2.to_position_index(&BoardSize::THIRTEEN), Some(175));
    assert_eq!(intsc2.to_position_index(&BoardSize::NINETEEN), Some(367));

    assert_eq!(intsc3.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc3.to_position_index(&BoardSize::THIRTEEN), Some(69));
    assert_eq!(intsc3.to_position_index(&BoardSize::NINETEEN), Some(219));

    assert_eq!(intsc4.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc4.to_position_index(&BoardSize::THIRTEEN), Some(42));
    assert_eq!(intsc4.to_position_index(&BoardSize::NINETEEN), Some(180));

    assert_eq!(intsc5.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc5.to_position_index(&BoardSize::THIRTEEN), None);
    assert_eq!(intsc5.to_position_index(&BoardSize::NINETEEN), Some(331));

    assert_eq!(intsc6.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc6.to_position_index(&BoardSize::THIRTEEN), None);
    assert_eq!(intsc6.to_position_index(&BoardSize::NINETEEN), Some(107));

    assert_eq!(intsc7.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc7.to_position_index(&BoardSize::THIRTEEN), None);
    assert_eq!(intsc7.to_position_index(&BoardSize::NINETEEN), Some(102));

    assert_eq!(intsc8.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc8.to_position_index(&BoardSize::THIRTEEN), None);
    assert_eq!(intsc8.to_position_index(&BoardSize::NINETEEN), None);

    assert_eq!(intsc9.to_position_index(&BoardSize::NINE), None);
    assert_eq!(intsc9.to_position_index(&BoardSize::THIRTEEN), None);
    assert_eq!(intsc9.to_position_index(&BoardSize::NINETEEN), None);
}

#[test]
fn test_intersection_from_position_index() {
    use ColumnIdentifier::*;

    // Offboard always results in None
    assert_eq!(Intersection::from_position_index(0, &BoardSize::NINE), None);
    assert_eq!(
        Intersection::from_position_index(0, &BoardSize::THIRTEEN),
        None
    );
    assert_eq!(
        Intersection::from_position_index(0, &BoardSize::NINETEEN),
        None
    );

    // Offboard on some Boards
    assert_eq!(
        Intersection::from_position_index(12, &BoardSize::NINE),
        Some(Intersection::new(A, 9))
    );
    assert_eq!(
        Intersection::from_position_index(12, &BoardSize::THIRTEEN),
        None // Offboard on 13x13
    );
    assert_eq!(
        Intersection::from_position_index(12, &BoardSize::NINETEEN),
        None // Offboard on 19x19
    );

    // Too large of position index returns None
    assert_eq!(
        Intersection::from_position_index(122, &BoardSize::NINE),
        None
    );
    assert_eq!(
        Intersection::from_position_index(122, &BoardSize::THIRTEEN),
        Some(Intersection::new(B, 6))
    );
    assert_eq!(
        Intersection::from_position_index(122, &BoardSize::NINETEEN),
        Some(Intersection::new(R, 15))
    );

    // Intersection that position index relates to depends on BoardSize
    assert_eq!(
        Intersection::from_position_index(73, &BoardSize::NINE),
        Some(Intersection::new(G, 4))
    );
    assert_eq!(
        Intersection::from_position_index(73, &BoardSize::THIRTEEN),
        Some(Intersection::new(N, 10))
    );
    assert_eq!(
        Intersection::from_position_index(73, &BoardSize::NINETEEN),
        Some(Intersection::new(K, 17))
    );
}

#[test]
fn test_opposite_color() {
    assert_eq!(Color::WHITE.opposite_color(), Color::BLACK);
    assert_eq!(Color::BLACK.opposite_color(), Color::WHITE);
}

// TODO: Potentially test RENDERING, currently omitted

/****************************************************\
|****************     GAME LOGIC     ****************|
\****************************************************/

#[test]
fn test_count() {
    use std::collections::HashSet;
    use ColumnIdentifier::*;
    let mut expected_group: HashSet<Intersection> = HashSet::new();
    let mut expected_liberties: HashSet<Intersection> = HashSet::new();
    let mut board = Board::new(BoardSize::NINETEEN);
    expected_group.insert(Intersection::new(B, 2));
    expected_group.insert(Intersection::new(C, 2));
    expected_group.insert(Intersection::new(D, 2));
    expected_group.insert(Intersection::new(D, 1));
    expected_group.insert(Intersection::new(E, 1));

    expected_liberties.insert(Intersection::new(B, 1));
    expected_liberties.insert(Intersection::new(C, 1));
    expected_liberties.insert(Intersection::new(A, 2));
    expected_liberties.insert(Intersection::new(D, 3));
    expected_liberties.insert(Intersection::new(F, 1));

    for intsc in &expected_group {
        board.play(Move::MOVE(intsc.clone(), Color::BLACK));
    }

    board.play(Move::MOVE(Intersection::new(F, 2), Color::BLACK));
    board.play(Move::MOVE(Intersection::new(B, 3), Color::WHITE));
    board.play(Move::MOVE(Intersection::new(C, 3), Color::WHITE));
    board.play(Move::MOVE(Intersection::new(E, 2), Color::WHITE));

    assert_eq!(
        board.count(
            Intersection::new(D, 1) // intersection does not matter as long as it is part of the group
                .to_position_index(&BoardSize::NINETEEN)
                .unwrap() as usize,
            Color::BLACK
        ),
        (expected_group, expected_liberties)
    );
}

#[test]
fn test_diamond_corner() {
    use ColumnIdentifier::*;
    let mut board = Board::new(BoardSize::NINETEEN);
    assert_eq!(board.diamond(&Intersection::new(A, 1)), None); // no diamond by default
    board.play(Move::MOVE(Intersection::new(A, 2), Color::WHITE));
    assert_eq!(board.diamond(&Intersection::new(A, 1)), None); // diamond not yet constructed
    board.play(Move::MOVE(Intersection::new(B, 1), Color::WHITE));
    assert_eq!(board.diamond(&Intersection::new(A, 1)), Some(Color::WHITE)); // diamond has been constructed on corner
}

#[test]
fn test_diamond_side() {
    use ColumnIdentifier::*;
    let mut board = Board::new(BoardSize::NINETEEN);
    assert_eq!(board.diamond(&Intersection::new(E, 1)), None); // no diamond by default
    board.play(Move::MOVE(Intersection::new(D, 1), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(E, 1)), None); // diamond not yet constructed
    board.play(Move::MOVE(Intersection::new(E, 2), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(E, 1)), None); // diamond still not yet constructed
    board.play(Move::MOVE(Intersection::new(F, 1), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(E, 1)), Some(Color::BLACK)); // diamond has been constructed on side
}

#[test]
fn test_diamond_center() {
    use ColumnIdentifier::*;
    let mut board = Board::new(BoardSize::NINETEEN);
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // no diamond by default
    board.play(Move::MOVE(Intersection::new(O, 14), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // diamond not yet constructed
    board.play(Move::MOVE(Intersection::new(O, 12), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // diamond still not yet constructed
    board.play(Move::MOVE(Intersection::new(N, 13), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // one more...
    board.play(Move::MOVE(Intersection::new(P, 13), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), Some(Color::BLACK)); // diamond has been constructed on side
}

#[test]
fn test_diamond_multiple_colors() {
    use ColumnIdentifier::*;
    let mut board = Board::new(BoardSize::NINETEEN);
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // no diamond by default
    board.play(Move::MOVE(Intersection::new(O, 14), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // diamond not yet constructed
    board.play(Move::MOVE(Intersection::new(O, 12), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // diamond still not yet constructed
    board.play(Move::MOVE(Intersection::new(N, 13), Color::BLACK));
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // one more...
    board.play(Move::MOVE(Intersection::new(P, 13), Color::WHITE)); // uh oh! opposite color present
    assert_eq!(board.diamond(&Intersection::new(O, 13)), None); // there is no diamond of a singular color
}

#[test]
fn test_play() {
    use ColumnIdentifier::*;
    let mut board = Board::new(BoardSize::NINETEEN);
    assert_eq!(board.play(Move::PASS), true);
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(E, 4), Color::BLACK)),
        true
    );
    // detailed play move testing done in test_play_intersection() for convenience
}

#[test]
fn test_play_intersection() {
    use ColumnIdentifier::*;
    let mut board = Board::new(BoardSize::NINE); // using BoardSize::NINE for out of bounds intersection tests
    let board_copy = board.deepcopy();
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(E, 0), Color::BLACK)),
        false
    );
    assert_eq!(board, board_copy); // board does not change on a failed play_intersection
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(A, 10), Color::BLACK)),
        false
    ); // too high of row for current BoardSize
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(K, 1), Color::BLACK)),
        false
    ); // too high of column for current BoardSize
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(O, 10), Color::BLACK)),
        false
    ); // both dimensions too high for current BoardSize

    assert_eq!(
        board.play(Move::MOVE(Intersection::new(E, 4), Color::BLACK)),
        true
    ); // play regular move
    assert_ne!(board, board_copy); // board does change after successful play_intersection
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(E, 4), Color::BLACK)),
        false
    ); // cannot play on occupied square with same color
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(E, 4), Color::WHITE)),
        false
    ); // cannot play on occupied square with opposite color

    // setup moves for Ko

    assert!(board.play(Move::MOVE(Intersection::new(F, 3), Color::BLACK))); // can play multiple moves by same color with no issue
    board.play(Move::MOVE(Intersection::new(G, 4), Color::BLACK));
    board.play(Move::MOVE(Intersection::new(F, 5), Color::BLACK));
    board.play(Move::MOVE(Intersection::new(E, 5), Color::WHITE));
    board.play(Move::MOVE(Intersection::new(F, 6), Color::WHITE));
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(F, 4), Color::WHITE)),
        false
    ); // cannot play suicidal moves
    board.play(Move::MOVE(Intersection::new(G, 5), Color::WHITE));

    assert!(board.play(Move::MOVE(Intersection::new(F, 4), Color::WHITE))); // capture checks come before suicide
    assert_eq!(board.white_captures, 1); // captures are correctly updated
    assert_eq!(
        board.play(Move::MOVE(Intersection::new(F, 5), Color::BLACK)),
        false
    ); // cannot play in Ko

    board.play(Move::MOVE(Intersection::new(A, 1), Color::BLACK));
    assert!(board.play(Move::MOVE(Intersection::new(F, 5), Color::BLACK))); // ko no longer exists after some other move
}
