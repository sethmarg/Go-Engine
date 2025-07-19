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
    NINE = 9,
    THIRTEEN = 13,
    NINETEEN = 19,
}

// Moves performed on a Board
enum Move {
    PASS,
    MOVE(Intersection, Color),
}

// Go Board structure
struct Board {
    size: u16,
    position: Vec<State>,
    side: Color,
    ko: Option<Intersection>,
    komi: f32,
    last_move: Move,
}

#[derive(PartialEq, Debug, Eq, Hash)]
enum ColumnIdentifier {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    J = 8,
    K = 9,
    L = 10,
    M = 11,
    N = 12,
    O = 13,
    P = 14,
    Q = 15,
    R = 16,
    S = 17,
    T = 18,
}

#[derive(PartialEq, Debug)]
#[derive(Eq, Hash)]
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
            size: numeric_size, // remember to account for OFFBOARD
            position: Board::empty_board(numeric_size),
            side: Color::BLACK,
            ko: Option::None,
            komi: 6.5,
            last_move: Move::PASS,
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
    fn to_u16(self) -> u16 {
        self as u16
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
    fn to_u16(self) -> u16 {
        self as u16
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
    fn to_position_index(self, size: BoardSize) -> u16 {
        let position_length = size.to_u16() + 2;
        let column_index = self.column.to_u16();
        let row_index = (position_length - self.row - 1) * position_length;
        column_index + row_index + 1
    }

    fn from_position_index(position_index: u16, size: BoardSize) -> Option<Intersection> {
        let position_length = size.to_u16() + 2;

        if (position_index >= position_index * position_index) {
            return Option::None;
        }

        let col = position_index % position_length;
        let row = position_index / position_length;

        if (col == 0 || col == position_length - 1 || row == 0 || row == position_length - 1) {
            return Option::None;
        }

        Some(Intersection {
            column: ColumnIdentifier::from_u16(col - 1).unwrap(),
            row: (position_length - row),
        })
    }
}

/*****************************************************\
|****************      RENDERING      ****************|
\*****************************************************/

impl Board {
    // Returns a String representing a rendering of the current Board
    fn render(&self) {
        let position_length = (self.size + 2) as usize;
        for row in 0..self.size as usize {
            print!("{} ", self.size as usize - row);
            for col in 0..self.size as usize {
                let intersection = (row + 1) * position_length + col + 1;
                match (self.position[intersection]) {
                    State::OCCUPIED(Color::BLACK) => print!("X "),
                    State::OCCUPIED(Color::WHITE) => print!("0 "),
                    State::EMPTY => print!(". "),
                    State::OFFBOARD => print!(""),
                }
            }
            println!()
        }

        print!(" ");
        for col in 0..self.size {
            print!(" {}", ColumnIdentifier::from_u16(col).unwrap());
        }
        println!();
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
            MOVE(intersection, color) => self.playIntersection(intersection, color),
        }
    }

    fn count(&self, position_index: usize, color: Color) -> (HashSet<Intersection>, HashSet<Intersection>) {
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
        let intsc = Intersection::from_position_index(
            position_index as u16,
            BoardSize::from_u16(self.size).unwrap(),
        )
        .unwrap();
        match intsc_state {
            State::OCCUPIED(intsc_color) => {
                if intsc_color == color {
                    if !group.contains(&intsc) {
                        group.insert(intsc);
                        self.count_help(position_index + 1, color, group, liberties);
                        self.count_help(position_index - 1, color, group, liberties);
                        self.count_help(
                            position_index + self.size as usize + 2,
                            color,
                            group,
                            liberties,
                        );
                        self.count_help(
                            position_index - self.size as usize - 2,
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

    fn play_intersection(&mut self, intsc: Intersection, color: Color) -> bool {
        if let Some(ko) = self.ko.as_ref() {
            if ko == &intsc {
                return false;
            }
        }

        let position_index =
            intsc.to_position_index(BoardSize::from_u16(self.size).unwrap()) as usize;
        if (self.position[position_index] != State::EMPTY) {
            return false;
        }

        self.position[position_index] = State::OCCUPIED(color);

        true
    }
}

fn main() {
    use ColumnIdentifier::*;
    let mut b: Board = Board::new(BoardSize::NINE);
    b.position[Intersection { column: B, row: 2 }.to_position_index(BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.position[Intersection { column: B, row: 3 }.to_position_index(BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.position[Intersection { column: C, row: 2 }.to_position_index(BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.position[Intersection { column: C, row: 3 }.to_position_index(BoardSize::NINE) as usize] =
        State::OCCUPIED(Color::WHITE);
    b.render();
    let (group, liberties) = b.count(
        Intersection { column: B, row: 2 }.to_position_index(BoardSize::NINE) as usize,
        Color::WHITE,
    );
    println!("Group: {:#?}", group);
    println!("Liberties: {:#?}", liberties);
}
