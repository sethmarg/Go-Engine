use num_traits::{Bounded, NumCast, Signed, Unsigned};
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};

/****************************************************\
|****************    GLOBAL TYPES    ****************|
\****************************************************/

// Stone colors
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Color {
    WHITE,
    BLACK,
}

// The state that a given intersection can be in
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum State {
    OCCUPIED(Color),
    EMPTY,
    OFFBOARD,
}

// Valid Go board sizes and their numeric values
#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum BoardSize {
    NINE,
    THIRTEEN,
    NINETEEN,
}

// Moves performed on a Board
#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Move {
    PASS,
    MOVE(Intersection, Color),
    RESIGN,
}

// Go Board structure
#[derive(PartialEq)]
pub(crate) struct Board {
    pub(crate) size: BoardSize,
    position: Vec<State>,
    side: Color,
    ko: Option<Intersection>,
    pub(crate) komi: f64,
    pub(crate) last_move: Move,
    pub(crate) white_captures: u16,
    pub(crate) black_captures: u16,
    pub(crate) move_number: u16,
}

// Identifiers of columns on the Go Board, used primarily for position notation
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub(crate) enum ColumnIdentifier {
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

// A structure that represents playable intersections on the Go Board
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy)]
pub(crate) struct Intersection {
    column: ColumnIdentifier,
    row: u16,
}

/*****************************************************\
|****************    PRIVATE TYPES    ****************|
\*****************************************************/

// Three state Option, where Yes is analogous to Some, No to None, and Unknown for a non-set state
enum Tristate<T> {
    Unknown,
    Yes(T),
    No,
}

impl<T> Tristate<T> {
    // Returns true if there is something known about this value (not Unknown)
    fn is_known(&self) -> bool {
        match self {
            Tristate::Unknown => false,
            Tristate::Yes(_) => true,
            Tristate::No => true,
        }
    }

    // Returns true if this Tristate is Yes
    fn is_yes(&self) -> bool {
        match self {
            Tristate::Yes(_) => true,
            _ => false,
        }
    }

    // Returns true if this Tristate is No
    fn is_no(&self) -> bool {
        match self {
            Tristate::No => true,
            _ => false,
        }
    }

    // Attempts to retrieve the value stored inside this Tristate.
    // Panics if impossible
    fn unwrap(self) -> T {
        match self {
            Tristate::Yes(value) => value,
            _ => panic!("Cannot unwrap Tristate that is not Tristate::Yes"),
        }
    }
}

/*****************************************************\
|****************        SETUP        ****************|
\*****************************************************/

impl Board {
    // Creates a new empty Board
    pub(crate) fn new(size: BoardSize) -> Board {
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
            move_number: 0,
        }
    }

    // Creates a Vec<State> representing an empty Go board
    fn empty_board(size: u16) -> Vec<State> {
        let mut position: Vec<State> = vec![];
        for row in 0..size + 2 {
            for col in 0..size + 2 {
                if row == 0 || row == size + 1 || col == 0 || col == size + 1 {
                    position.push(State::OFFBOARD);
                } else {
                    position.push(State::EMPTY);
                }
            }
        }

        position
    }

    // Creates and returns a new identical Board to this one
    // which has no aliasing nor relation to this Board
    pub(crate) fn deepcopy(&self) -> Board {
        let mut position_copy: Vec<State> = vec![];
        for intsc_state in &self.position {
            position_copy.push(intsc_state.clone());
        }

        Board {
            size: self.size.clone(),
            position: position_copy,
            side: self.side.clone(),
            ko: self.ko.clone(),
            komi: self.komi.clone(),
            last_move: self.last_move.clone(),
            white_captures: self.white_captures,
            black_captures: self.black_captures,
            move_number: self.move_number,
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: try to maybe find a way to add the /n before the }
        f.debug_struct("Board")
            .field("\n\tSize", &self.size)
            .field("\n\tPosition", &self.position)
            .field("\n\tKo", &self.ko)
            //.field("\n\tKomi", &self.komi)
            .field("\n\tLast Move", &self.last_move)
            .field("\n\tWhite Captures", &self.white_captures)
            .field("\n\tBlack Captures", &self.black_captures)
            .finish()
    }
}

// Represents an intersection on the Go board
impl Intersection {
    // Creates a new Intersection with the given column and row
    pub(crate) fn new(column: ColumnIdentifier, row: u16) -> Intersection {
        Intersection { column, row }
    }
}

/****************************************************\
|****************       HELPER       ****************|
\****************************************************/

// Adds the given i16 value to the base usize value.
// If an underflow or overflow occurs, returns None.
// Else, returns Some(sum as usize)
pub(crate) fn add_signed_to_unsigned<U, S>(base: U, to_add: S) -> Option<U>
where
    U: Unsigned
        + Copy
        + Add<Output = U>
        + Sub<Output = U>
        + Bounded
        + NumCast
        + std::cmp::PartialOrd,
    S: Signed + Copy + NumCast + std::cmp::PartialOrd,
{
    if to_add >= S::zero() {
        let add_u: U = NumCast::from(to_add)?;
        if U::max_value() - add_u < base {
            return None;
        }
        Some(base + add_u)
    } else {
        let sub_u: U = NumCast::from(to_add.abs())?;
        if sub_u > base {
            return None;
        }
        Some(base - sub_u)
    }
}

impl BoardSize {
    // converts numeric board sizes into their respective BoardSize
    pub(crate) fn from_u16(size: u16) -> Option<BoardSize> {
        match size {
            9 => Some(BoardSize::NINE),
            13 => Some(BoardSize::THIRTEEN),
            19 => Some(BoardSize::NINETEEN),
            _ => None,
        }
    }

    // Converts a BoardSize to its equivalent numeric value
    pub(crate) fn to_u16(&self) -> u16 {
        match self {
            BoardSize::NINE => 9,
            BoardSize::THIRTEEN => 13,
            BoardSize::NINETEEN => 19,
        }
    }
}

impl ColumnIdentifier {
    // Converts numeric column indices to their respective ColumnIdentifier
    // TODO: seems messy, likely cleaner way to do this
    pub(crate) fn from_u16(column_index: u16) -> Option<ColumnIdentifier> {
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

    // Attempts to convert the given string into a ColumnIdentifier
    // Returns a Some() with the identifier if successful, else returns None
    pub(crate) fn from_string(string: &str) -> Option<ColumnIdentifier> {
        use ColumnIdentifier::*;
        match string.to_uppercase().as_str() {
            "A" => Some(A),
            "B" => Some(B),
            "C" => Some(C),
            "D" => Some(D),
            "E" => Some(E),
            "F" => Some(F),
            "G" => Some(G),
            "H" => Some(H),
            "J" => Some(J),
            "K" => Some(K),
            "L" => Some(L),
            "M" => Some(M),
            "N" => Some(N),
            "O" => Some(O),
            "P" => Some(P),
            "Q" => Some(Q),
            "R" => Some(R),
            "S" => Some(S),
            "T" => Some(T),
            _ => None,
        }
    }

    // Converts a ColumnIdentifier to its respective u16 column index
    pub(crate) fn to_u16(&self) -> u16 {
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
    pub(crate) fn to_position_index(&self, size: &BoardSize) -> Option<u16> {
        let position_length = size.to_u16() + 2;
        let column_index = self.column.to_u16();
        if column_index >= size.to_u16() || self.row > size.to_u16() || self.row == 0 {
            None
        } else {
            // row_index must be defined here in now impossible the event self.row is 0
            let row_index = (position_length - self.row - 1) * position_length;
            Some(column_index + row_index + 1)
        }
    }

    // Given a position index on a Board, returns the Intersection that correlates to the
    // index if valid for the given BoardSize. Else, returns None
    pub(crate) fn from_position_index(
        position_index: u16,
        size: &BoardSize,
    ) -> Option<Intersection> {
        let position_length = size.to_u16() + 2;

        if position_index >= position_length * position_length {
            return None;
        }

        let col = position_index % position_length;
        let row = position_index / position_length;

        if col == 0 || col == position_length - 1 || row == 0 || row == position_length - 1 {
            return None;
        }

        Some(Intersection {
            column: ColumnIdentifier::from_u16(col - 1).unwrap(),
            row: position_length - row - 1,
        })
    }

    // Attempts to convert the given string into an Intersection
    // Returns a Some() with the Intersection if successful, else returns None
    // (This method is successful if the format of the String is correct,
    // even if the Intersection returned is ridiculous)
    pub(crate) fn from_string(string: &str) -> Option<Intersection> {
        if string.len() < 2 {
            return None;
        }

        let col = &string[0..1];
        let row = &string[1..];

        if ColumnIdentifier::from_string(col).is_none() {
            return None;
        }

        if row.parse::<u16>().is_err() {
            return None;
        }

        Some(Intersection {
            column: ColumnIdentifier::from_string(col).unwrap(),
            row: row.parse().unwrap(),
        })
    }
}

impl Color {
    // Returns the opposite color of the current
    pub(crate) fn opposite_color(&self) -> Color {
        match self {
            Color::WHITE => Color::BLACK,
            Color::BLACK => Color::WHITE,
        }
    }

    // Attempts to convert the given string into a Color
    // Returns a Some() with the Color if successful, else returns None
    pub(crate) fn from_string(string: &str) -> Option<Color> {
        match string.to_lowercase().as_str() {
            "b" | "black" => Some(Color::BLACK),
            "w" | "white" => Some(Color::WHITE),
            _ => None,
        }
    }
}

/*****************************************************\
|****************      RENDERING      ****************|
\*****************************************************/

impl fmt::Display for Board {
    // TODO: MESSY CODE PLEASE FIX AND MAKE MORE READABLE
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut render: String = String::from("");
        let position_length = (self.size.to_u16() + 2) as usize;
        for row in 1..position_length - 1 {
            if position_length - row <= 10 {
                // TODO: fix this
                render = format!("{render} ");
            }
            render = format!("{render}{} ", position_length - row - 1);
            for col in 1..position_length - 1 {
                let intersection = row * position_length + col; // TODO: also this
                match self.position[intersection] {
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
        render = format!("{render}\nKomi:     {}", self.komi);
        render = format!(
            "{render}\nKo:       {}",
            match self.ko {
                Some(intersection) => intersection.to_string(),
                None => "None".to_string(),
            }
        );
        render = format!(
            "{render}\nCaptures: [B: {}, W: {}]",
            self.black_captures, self.white_captures
        );

        write!(f, "\n{render}\n")
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.column.to_string(), self.row)
    }
}

/****************************************************\
|****************     GAME LOGIC     ****************|
\****************************************************/

impl Board {
    // For a group of stones starting at the given position_index,
    // returns a tuple of HashSet<Intersections> containing the stones in the group
    // and the group's liberties respectively
    pub(crate) fn count(
        &self,
        position_index: usize,
        color: Color,
    ) -> (HashSet<Intersection>, HashSet<Intersection>) {
        let mut group: HashSet<Intersection> = HashSet::new();
        let mut liberties: HashSet<Intersection> = HashSet::new();

        self.count_help(position_index, color, &mut group, &mut liberties);

        (group, liberties)
    }

    // Recursive helper for the above count function
    fn count_help(
        &self,
        position_index: usize,
        color: Color,
        group: &mut HashSet<Intersection>,
        liberties: &mut HashSet<Intersection>,
    ) {
        // TODO: likely should be broken up into more helpers
        let intsc_state = self.position[position_index];
        let intsc = Intersection::from_position_index(position_index as u16, &self.size);
        match intsc_state {
            State::OCCUPIED(intsc_color) => {
                if intsc_color == color {
                    let intsc_unwrapped = intsc.unwrap();
                    if !group.contains(&intsc_unwrapped) {
                        group.insert(intsc_unwrapped);
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
                let intsc_unwrapped = intsc.unwrap();
                if !liberties.contains(&intsc_unwrapped) {
                    liberties.insert(intsc_unwrapped);
                }
            }
            State::OFFBOARD => {} // do nothing
        }
    }

    // Captures the stones found in the given group, setting each intersection in the board position
    // to State::EMPTY and incrementing the appropriate Board capture field by the number of
    // stones captured
    pub(crate) fn capture_group(&mut self, group: HashSet<Intersection>, color: Color) {
        let stones = group.len() as u16;

        for intsc in group {
            if let Some(stone) = intsc.to_position_index(&self.size) {
                self.position[stone as usize] = State::EMPTY;
            }
        }

        match color {
            Color::WHITE => self.white_captures += stones,
            Color::BLACK => self.black_captures += stones,
        }
    }

    // If there is a diamond shape completely surrounding the given Intersection on this Board,
    // return an Option containing its color. Else, return None
    pub(crate) fn diamond(&self, intsc: &Intersection) -> Option<Color> {
        if let Some(position_index) = intsc.to_position_index(&self.size) {
            let mut diamond_color: Option<Color> = None;
            let numeric_size = self.size.to_u16() as i16;

            for dir in [1, -1, numeric_size + 2, -numeric_size - 2] {
                let surrounding_position_index =
                    add_signed_to_unsigned(position_index as usize, dir);
                if surrounding_position_index.is_some() {
                    match self.position[surrounding_position_index.unwrap()] {
                        State::EMPTY => return None,
                        State::OCCUPIED(color) => match diamond_color {
                            Some(cur_color) => {
                                if cur_color != color {
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

    // Attempts to play the given Move on this Board. If successful, updates the current Board
    // accordingly and returns true. Else returns false.
    pub(crate) fn play(&mut self, mov: Move) -> bool {
        use Move::*;
        let was_move_played = match mov {
            PASS => true,
            MOVE(intersection, color) => self.play_intersection(intersection, color),
            RESIGN => false,
        };
        
        if was_move_played {
            self.move_number += 1;
        }
        
        was_move_played
    }

    // Attempts to play a stone of the given Color and the given Intersection on this Board.
    // If successful, updates this Board accordingly and returns true. Else returns false.
    fn play_intersection(&mut self, intsc: Intersection, color: Color) -> bool {
        //TODO: absolutely update this function with helpers, completely unreadable in current state
        if let Some(ko) = self.ko.as_ref() {
            if ko == &intsc {
                return false;
            }
        }

        if let Some(position_index_u16) = intsc.to_position_index(&self.size) {
            let position_index = position_index_u16 as usize;
            if self.position[position_index] != State::EMPTY {
                return false;
            }

            let mut new_ko: Option<Intersection> = None;

            self.position[position_index] = State::OCCUPIED(color);

            // capture logic
            let numeric_size = self.size.to_u16() as i16;
            for dir in [1, -1, numeric_size + 2, -numeric_size - 2] {
                if let Some(surrounding_intsc_index) = add_signed_to_unsigned(position_index, dir) {
                    let (group, liberties) =
                        self.count(surrounding_intsc_index, color.opposite_color());
                    if liberties.len() == 0 {
                        if group.len() == 1 {
                            // ensures not OFFBOARD for diamond check
                            let surrounding_intsc = Intersection::from_position_index(
                                surrounding_intsc_index as u16,
                                &self.size,
                            )
                            .unwrap();
                            if let Some(surrounding_color) = self.diamond(&intsc) {
                                if surrounding_color != color {
                                    new_ko = Some(surrounding_intsc);
                                }
                            }
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
            self.side = color.opposite_color();
            self.last_move = Move::MOVE(intsc, color);

            return true;
        };
        false
    }
}

/*******************************************************\
|****************        SCORING        ****************|
\*******************************************************/

impl Board {
    // Estimates the score at the end of the Go game on this Board
    pub(crate) fn estimate_score(&self) -> f64 {
        let mut intsc_seen: HashSet<Intersection> = HashSet::new();
        let mut reaches_black: i16 = 0;
        let mut reaches_white: i16 = 0;

        for row in 0..self.size.to_u16() {
            for col in 0..self.size.to_u16() {
                let intsc = Intersection::new(ColumnIdentifier::from_u16(col).unwrap(), row + 1);
                if !intsc_seen.contains(&intsc) {
                    let (intersections, reaches_color) = self.tromp_taylor_count(intsc);
                    if reaches_color.is_yes() {
                        match reaches_color.unwrap() {
                            Color::BLACK => reaches_black += intersections.len() as i16,
                            Color::WHITE => reaches_white += intersections.len() as i16,
                        }
                    }
                    intsc_seen.extend(intersections);
                }
            }
        }

        (reaches_black - reaches_white) as f64 - self.komi
    }

    fn tromp_taylor_count(
        &self,
        root_intsc: Intersection,
    ) -> (HashSet<Intersection>, Tristate<Color>) {
        use Tristate::*;
        let mut intsc_seen: HashSet<Intersection> = HashSet::new();
        let mut reaches_color: Tristate<Color> = Unknown;
        let mut work_list: VecDeque<Intersection> = VecDeque::new();
        work_list.push_back(root_intsc);

        while !work_list.is_empty() {
            let intsc = work_list.pop_front().unwrap(); // work_list is not empty, safe
            if !intsc_seen.contains(&intsc) {
                let intsc_index = intsc.to_position_index(&self.size).unwrap(); // later logic ensures safety
                let intsc_state = self.position[intsc_index as usize];

                match intsc_state {
                    State::OFFBOARD => {}
                    State::OCCUPIED(color) => {
                        reaches_color = match reaches_color {
                            Unknown => Yes(color),
                            Yes(reached_color) => {
                                if color == reached_color {
                                    Yes(color)
                                } else {
                                    No
                                }
                            }
                            No => No,
                        }
                    }
                    State::EMPTY => {
                        work_list.extend(self.neighboring_intersections(&intsc));
                    }
                }

                intsc_seen.insert(intsc);
            }
        }

        (intsc_seen, reaches_color)
    }

    fn neighboring_intersections(&self, intsc: &Intersection) -> Vec<Intersection> {
        let mut neighbors: Vec<Intersection> = vec![];
        if let Some(index) = intsc.to_position_index(&self.size) {
            let numeric_size = self.size.to_u16() as i16;
            for dir in [1, -1, numeric_size + 2, -numeric_size - 2] {
                if let Some(neighbor) = Intersection::from_position_index(
                    add_signed_to_unsigned(index, dir).unwrap(),
                    &self.size,
                ) {
                    neighbors.push(neighbor);
                }
            }
        }

        neighbors
    }
}

/*******************************************************\
|****************  PLAYOUT (TEMPORARY)  ****************|
\*******************************************************/

impl Board {
    // Finds the weakest group on the board, and returns its liberties
    // todo: TEMP METHOD, perhaps add better logic
    pub(crate) fn weakest_group(&self, color: &Color) -> Vec<Intersection> {
        let mut liberties: Vec<Intersection> = vec![];
        let mut smallest_size: usize = 1000; // actually needed for when liberties is empty as init
        let mut intsc_seen: HashSet<Intersection> = HashSet::new();

        for index in 0..self.position.len() {
            if self.position[index] == State::OCCUPIED(*color) {
                let intsc = Intersection::from_position_index(index as u16, &self.size).unwrap();
                if !intsc_seen.contains(&intsc) {
                    let (group, group_libs) = self.count(index, *color);

                    for group_intsc in group {
                        intsc_seen.insert(group_intsc);
                    }

                    if group_libs.len() < smallest_size {
                        smallest_size = group_libs.len();
                        liberties = group_libs.into_iter().collect();
                    }
                }
            }
        }

        liberties
    }

    // Returns a random intersection found on this Board
    // todo: TEMP METHOD
    pub(crate) fn random_intersection(&self, offset: u16) -> Intersection {
        use rand::Rng;
        let mut moves: Vec<Intersection> = vec![];
        for row in 1 + offset..self.size.to_u16() - offset {
            for col in 1 + offset..self.size.to_u16() - offset {
                let col_iden = ColumnIdentifier::from_u16(col).unwrap();
                moves.push(Intersection::new(col_iden, row));
            }
        }

        let ind = rand::thread_rng().gen_range(0..moves.len());
        moves[ind]
    }

    // Ensures playing a stone at this position is not suicide
    // todo: TEMP METHOD AND ITS ALSO REALLY BAD
    pub(crate) fn not_suicide(&self, intsc: &Intersection) -> bool {
        if let Some(position_index) = intsc.to_position_index(&self.size) {
            let mut liberties = 0;
            let numeric_size = self.size.to_u16() as i16;
            for dir in [1, -1, numeric_size, -numeric_size] {
                let neighbor = add_signed_to_unsigned(position_index as usize, dir);
                if neighbor.is_some() && self.position[neighbor.unwrap()] == State::EMPTY {
                    liberties += 1;
                }
            }

            self.position[position_index as usize] == State::EMPTY
                && liberties > 0
                && Some(intsc) != self.ko.as_ref()
        } else {
            false
        }
    }

    // Is it possible to place a stone at the given Intersection on this Board?
    pub(crate) fn can_place_stone_at(&self, intsc: &Intersection) -> bool {
        if let Some(position_index) = intsc.to_position_index(&self.size) {
            self.position[position_index as usize] == State::EMPTY && self.not_suicide(intsc)
        } else {
            false
        }
    }
}
