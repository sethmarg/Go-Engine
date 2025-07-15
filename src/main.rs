use crate::ColumnIdentifier::*;
/****************************************************\
|****************    GLOBAL TYPES    ****************|
\****************************************************/
// Stone colors
#[derive(Copy, Clone)]
enum Color {
    WHITE,
    BLACK,
}

// The state that a given intersection can be in
#[derive(Copy, Clone)]
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
    size: u8,
    position: Vec<State>,
    side: Color,
    ko: Move,
    komi: f32,
    last_move: Move,
}

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

struct Intersection {
    column: ColumnIdentifier,
    row: u8,
}

/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

impl Board {
    // Creates a new empty Board
    fn new(size: BoardSize) -> Board {
        let numeric_size = size.to_u8();
        Board {
            size: numeric_size, // remember to account for OFFBOARD
            position: Board::empty_board(numeric_size),
            side: Color::BLACK,
            ko: Move::PASS,
            komi: 6.5,
            last_move: Move::PASS,
        }
    }

    // Creates a Vec<State> representing an empty Go board
    fn empty_board(size: u8) -> Vec<State> {
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
    fn from_u8(size: u8) -> Option<BoardSize> {
        match size {
            9 => Some(BoardSize::NINE),
            13 => Some(BoardSize::THIRTEEN),
            19 => Some(BoardSize::NINETEEN),
            _ => None,
        }
    }

    // Converts a BoardSize to its equivalent numeric value
    fn to_u8(self) -> u8 {
        self as u8
    }
}

impl ColumnIdentifier {
    // Converts numeric column indecies to their respective ColumnIdentifier
    // TODO: seems messy, likely cleaner way to do this
    fn from_u16(column_index: u16) -> Option<ColumnIdentifier> {
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

impl Intersection {
    fn to_position_index(self, size: BoardSize) -> usize {
        let numeric_size = size.to_u8();
        let row_index = ((numeric_size - self.row - 1) * numeric_size) as u16;
        (row_index + self.column.to_u16()) as usize
    }
}

/****************************************************\
|****************     GAME LOGIC     ****************|
\****************************************************/

impl Board {
    fn play(mut self, mov: Move) -> Board {
        match mov {
            Move::PASS => self,
            Move::MOVE(intersection, color) => {
                let position_index =
                    intersection.to_position_index(BoardSize::from_u8(self.size).unwrap());
                self.position[position_index] = State::OCCUPIED(color);
                self
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
