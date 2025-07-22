#[cfg(test)]
use super::*;

/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

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
    let intsc1 = Intersection { column: A, row: 1 }; // works on every board
    let intsc2 = Intersection { column: K, row: 3 }; // does not work on 9x9, too high of column
    let intsc3 = Intersection { column: J, row: 10 }; // does not work on 9x9, too high of row
    let intsc4 = Intersection { column: M, row: 12 }; // does not work on 9x9, both dimensions
    let intsc5 = Intersection { column: Q, row: 5 }; // only works on 19x19, too high of column
    let intsc6 = Intersection { column: B, row: 15 }; // only works on 19x19, too high of row
    let intsc7 = Intersection { column: S, row: 16 }; // only works on 19x19, both dimensions
    let intsc8 = Intersection { column: A, row: 20 }; // never works
    let intsc9 = Intersection { column: O, row: 0 }; // too low of row, never works

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
        Some(Intersection { column: A, row: 9 })
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
        Some(Intersection { column: B, row: 6 })
    );
    assert_eq!(
        Intersection::from_position_index(122, &BoardSize::NINETEEN),
        Some(Intersection { column: R, row: 15 })
    );

    // Intersection that position index relates to depends on BoardSize
    assert_eq!(
        Intersection::from_position_index(73, &BoardSize::NINE),
        Some(Intersection { column: G, row: 4 })
    );
    assert_eq!(
        Intersection::from_position_index(73, &BoardSize::THIRTEEN),
        Some(Intersection { column: N, row: 10})
    );
    assert_eq!(
        Intersection::from_position_index(73, &BoardSize::NINETEEN),
        Some(Intersection {column: K, row: 17})
    );
}

#[test]
fn test_opposite_color() {
    assert_eq!(Color::WHITE.opposite_color(), Color::BLACK);
    assert_eq!(Color::BLACK.opposite_color(), Color::WHITE);
}