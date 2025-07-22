use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;

/****************************************************\
|****************    GLOBAL TYPES    ****************|
\****************************************************/

// Stone colors
#[derive(Copy, Clone, PartialEq)]
enum Color {
    WHITE,
    BLACK,
}

// The state that a given intersection can be in
#[derive(Copy, Clone, PartialEq)]
enum State {
    OCCUPIED(Color),
    EMPTY,
    OFFBOARD,
}

// Valid Go board sizes and their numeric values
enum BoardSize {
    NINE,
    THIRTEEN,
    NINETEEN,
}

// Moves performed on a Board
enum Move {
    PASS,
    MOVE(Intersection, Color),
}

// Go Board structure
struct Board {
    size: BoardSize,
    position: Vec<State>,
    side: Color,
    ko: Option<Intersection>,
    komi: f32,
    last_move: Move,
    white_captures: u16,
    black_captures: u16,
}

#[derive(PartialEq, Debug, Eq, Hash)]
enum ColumnIdentifier {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
}

#[derive(PartialEq, Debug, Eq, Hash)]
struct Intersection {
    column: ColumnIdentifier,
    row: u16,
}

/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

impl Board {
    // Creates a new empty Board
    fn new(size: BoardSize) -> Board {
        let numeric_size = size.to_u16();
        Board {
            size,
            position: Board::empty_board(numeric_size),
            side: Color::BLACK,
            ko: None,
            komi: 6.5,
            last_move: Move::PASS,
            white_captures: 0,
            black_captures: 0,
        }
    }

    // Creates a Vec<State> representing an empty Go board
    fn empty_board(size: u16) -> Vec<State> {
        let mut position: Vec<State> = vec![];
        for row in 0..size + 2 {
            for col in 0..size + 2 {
                if (row == 0 || row == size + 1 || col == 0 || col == size + 1) {
                    position.push(State::OFFBOARD);
                } else {
                    position.push(State::EMPTY);
                }
            }
        }

        position
    }
}

/****************************************************\
|****************       HELPER       ****************|
\****************************************************/

// Adds the given i16 value to the base usize value.
// If an underflow or overflow occurs, returns None.
// Else, returns Some(sum as usize)
fn add_to_usize(base: usize, to_add: i16) -> Option<usize> {
    if to_add > 0 {
        if (usize::MAX - to_add as usize) < base {
            return None;
        }
        Some(base + to_add as usize)
    } else {
        if to_add.abs() as usize > base {
            return None;
        }
        Some(base - to_add.abs() as usize)
    }
}

impl BoardSize {
    // converts numeric board sizes into their respective BoardSize
    fn from_u16(size: u16) -> Option<BoardSize> {
        match size {
            9 => Some(BoardSize::NINE),
            13 => Some(BoardSize::THIRTEEN),
            19 => Some(BoardSize::NINETEEN),
            _ => None,
        }
    }

    // Converts a BoardSize to its equivalent numeric value
    fn to_u16(&self) -> u16 {
        match self {
            BoardSize::NINE => 9,
            BoardSize::THIRTEEN => 13,
            BoardSize::NINETEEN => 19,
        }
    }
}

impl ColumnIdentifier {
    // Converts numeric column indecies to their respective ColumnIdentifier
    // TODO: seems messy, likely cleaner way to do this
    fn from_u16(column_index: u16) -> Option<ColumnIdentifier> {
        use ColumnIdentifier::*;
        match column_index {
            0 => Some(A),
            1 => Some(B),
            2 => Some(C),
            3 => Some(D),
            4 => Some(E),
            5 => Some(F),
            6 => Some(G),
            7 => Some(H),
            8 => Some(J),
            9 => Some(K),
            10 => Some(L),
            11 => Some(M),
            12 => Some(N),
            13 => Some(O),
            14 => Some(P),
            15 => Some(Q),
            16 => Some(R),
            17 => Some(S),
            18 => Some(T),
            _ => None,
        }
    }

    // Converts a ColumnIdentifier to its respective u16 column index
    fn to_u16(&self) -> u16 {
        use ColumnIdentifier::*;
        match self {
            A => 0,
            B => 1,
            C => 2,
            D => 3,
            E => 4,
            F => 5,
            G => 6,
            H => 7,
            J => 8,
            K => 9,
            L => 10,
            M => 11,
            N => 12,
            O => 13,
            P => 14,
            Q => 15,
            R => 16,
            S => 17,
            T => 18,
        }
    }
}

impl fmt::Display for ColumnIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use ColumnIdentifier::*;
        write!(
            f,
            "{}",
            match self {
                A => "A",
                B => "B",
                C => "C",
                D => "D",
                E => "E",
                F => "F",
                G => "G",
                H => "H",
                J => "J",
                K => "K",
                L => "L",
                M => "M",
                N => "N",
                O => "O",
                P => "P",
                Q => "Q",
                R => "R",
                S => "S",
                T => "T",
            }
        )
    }
}

impl Intersection {
    // Converts this Intersection into its index in a position vector on the given BoardSize Board
    // TODO: TESTS!!!
    fn to_position_index(&self, size: &BoardSize) -> u16 {
        let position_length = size.to_u16() + 2;
        let column_index = self.column.to_u16();
        let row_index = (position_length - self.row - 1) * position_length;
        column_index + row_index + 1
    }

    fn from_position_index(position_index: u16, size: &BoardSize) -> Option<Intersection> {
        let position_length = size.to_u16() + 2;

        if position_index >= position_length * position_length {
            return None;
        }

        let col = position_index % position_length;
        let row = position_index / position_length;

        if (col == 0 || col == position_length - 1 || row == 0 || row == position_length - 1) {
            return None;
        }

        Some(Intersection {
            column: ColumnIdentifier::from_u16(col - 1).unwrap(),
            row: (position_length - row),
        })
    }
}

impl Color {
    // Returns the opposite color of the current
    fn oppositeColor(&self) -> Color {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }
}

/*****************************************************\
|****************      RENDERING      ****************|
\*****************************************************/

impl Board {
    // Returns a String representing a rendering of the current Board
    fn render(&self) -> String {
        let mut render: String = String::from("");
        let position_length = (self.size.to_u16() + 2) as usize;
        for row in 1..position_length - 1 {
            render = format!("{render}{} ", position_length - row - 1);
            for col in 1..position_length - 1 {
                let intersection = (row + 1) * position_length + col;
                match (self.position[intersection]) {
                    State::OCCUPIED(Color::BLACK) => render = format!("{render}X "),
                    State::OCCUPIED(Color::WHITE) => render = format!("{render}O "),
                    State::EMPTY => render = format!("{render}. "),
                    State::OFFBOARD => {}
                }
            }
            render = format!("{render}\n");
        }

        render = format!("{render}  ");
        for col in 1..position_length as u16 - 1 {
            render = format!("{render} {}", ColumnIdentifier::from_u16(col - 1).unwrap());
        }
        render + "\n"
    }
}

/****************************************************\
|****************     GAME LOGIC     ****************|
\****************************************************/

impl Board {
    fn play(&mut self, mov: Move) -> bool {
        use Move::*;
        match mov {
            PASS => true,
            MOVE(intersection, color) => self.play_intersection(intersection, color),
        }
    }

    fn count(
        &self,
        position_index: usize,
        color: Color,
    ) -> (HashSet<Intersection>, HashSet<Intersection>) {
        let mut group: HashSet<Intersection> = HashSet::new();
        let mut liberties: HashSet<Intersection> = HashSet::new();

        self.count_help(position_index, color, &mut group, &mut liberties);

        (group, liberties)
    }

    fn count_help(
        &self,
        position_index: usize,
        color: Color,
        group: &mut HashSet<Intersection>,
        liberties: &mut HashSet<Intersection>,
    ) {
        let intsc_state = self.position[position_index];
        let intsc = Intersection::from_position_index(position_index as u16, &self.size).unwrap();
        match intsc_state {
            State::OCCUPIED(intsc_color) => {
                if intsc_color == color {
                    if !group.contains(&intsc) {
                        group.insert(intsc);
                        self.count_help(position_index + 1, color, group, liberties);
                        self.count_help(position_index - 1, color, group, liberties);
                        self.count_help(
                            position_index + self.size.to_u16() as usize + 2,
                            color,
                            group,
                            liberties,
                        );
                        self.count_help(
                            position_index - self.size.to_u16() as usize - 2,
                            color,
                            group,
                            liberties,
                        );
                    }
                }
            }
            State::EMPTY => {
                if !liberties.contains(&intsc) {
                    liberties.insert(intsc);
                }
            }
            State::OFFBOARD => {} // do nothing
        }
    }

    // Captures the stones found in the given group, setting each intersection in the board position
    // to State::EMPTY and incrementing the appropriate Board capture field by the number of
    // stones captured
    fn capture_group(&mut self, group: HashSet<Intersection>, color: Color) {
        let stones = group.len() as u16;

        for intsc in group {
            self.position[intsc.to_position_index(&self.size) as usize] = State::EMPTY;
        }

        match color {
            Color::WHITE => self.white_captures += stones,
            Color::BLACK => self.black_captures += stones,
        }
    }

    // If there is a diamond shape completely surrounding the given Intersection on this Board,
    // return an Option containing its color. Else, return None
    fn diamond(&self, intsc: &Intersection) -> Option<Color> {
        let position_index = intsc.to_position_index(&self.size) as usize;
        let mut diamond_color: Option<Color> = None;
        let numeric_size = self.size.to_u16() as i16;

            for dir in [1, -1, numeric_size + 2, -numeric_size - 2] {
                let surrounding_position_index = add_to_usize(position_index as usize, dir);
                if surrounding_position_index.is_some() {
                    match self.position[surrounding_position_index.unwrap()] {
                        State::EMPTY => return None,
                        State::OCCUPIED(color) => match diamond_color {
                            Some(cur_color) => {
                                if (cur_color != color) {
                                    return None;
                                }
                            }
                            None => diamond_color = Some(color),
                        },
                        State::OFFBOARD => {}
                    }
                }
            }
            return diamond_color;
        };
        None
    }

    fn play_intersection(&mut self, intsc: Intersection, color: Color) -> bool {
        if let Some(ko) = self.ko.as_ref() {
            if ko == &intsc {
                return false;
            }
        }

        let position_index = intsc.to_position_index(&self.size) as usize;
        if (self.position[position_index] != State::EMPTY) {
            return false;
        }

            let mut new_ko: Option<Intersection> = None;

            self.position[position_index] = State::OCCUPIED(color);

            // capture logic
            let numeric_size = self.size.to_u16() as i16;
            println!("Index: {position_index}, Intersection: {:#?}", intsc);
            for dir in [1, -1, numeric_size + 2, -numeric_size - 2] {
                let o_surrounding_intsc_index = add_to_usize(position_index, dir);
                if o_surrounding_intsc_index.is_some() {
                    let surrounding_intsc_index = o_surrounding_intsc_index.unwrap();
                    let (group, liberties) =
                        self.count(surrounding_intsc_index, color.opposite_color());
                    if (liberties.len() == 0) {
                        if (group.len() == 1) {
                            // ensures not OFFBOARD for diamond check
                            let surrounding_intsc = Intersection::from_position_index(
                                surrounding_intsc_index as u16,
                                &self.size,
                            )
                                .unwrap();
                            let surrounding_color = self.diamond(&surrounding_intsc);
                            match surrounding_color {
                                Some(c) => {
                                    if c != color {
                                        new_ko = Some(surrounding_intsc);
                                    }
                                }
                                None => {}
                            };
                        }
                        self.capture_group(group, color);
                    }
                }
            }

            // ensure not suicide
            let (_, played_liberties) = self.count(position_index, color);
            if played_liberties.len() == 0 {
                self.position[position_index] = State::EMPTY;
                return false;
            }

            // move goes through
            self.ko = new_ko;
            self.side = self.side.opposite_color();
            self.last_move = Move::MOVE(intsc, color);

        true
    }
}

fn main() {
    use ColumnIdentifier::*;
    let mut b: Board = Board::new(BoardSize::NINETEEN);
    b.position[Intersection { column: B, row: 2 }.to_position_index(&BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.position[Intersection { column: B, row: 3 }.to_position_index(&BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.position[Intersection { column: C, row: 2 }.to_position_index(&BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.position[Intersection { column: C, row: 3 }.to_position_index(&BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    print!("{}", b.render());
    let (group, liberties) = b.count(
        Intersection { column: B, row: 2 }.to_position_index(&BoardSize::NINE) as usize,
        Color::WHITE,
    );
    println!("Group: {:#?}", group);
    println!("Liberties: {:#?}", liberties);
}
