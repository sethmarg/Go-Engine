// The state that a given intersection can be in
#[derive(Copy)]
#[derive(Clone)]
enum State {
    WHITE,
    BLACK,
    EMPTY,
    OFFBOARD,
}

// Valid Go board sizes and their numeric values
enum BoardSize {
    NINE = 9,
    THIRTEEN = 13,
    NINETEEN = 19,
}

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

// Moves performed on a Board
enum Move {
    PASS,
    MOVE(String),
}

// Go Board structure
struct Board {
    size: u8,
    position: Vec<State>,
    side: State,
    ko: Move,
    komi: f32,
    last_move: Move,
}

fn main() {
    println!("Hello, world!");
}