use super::*;
use board::*;
use std::fmt::Formatter;
use std::{fmt, io};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
/****************************************************\
|****************    GLOBAL TYPES    ****************|
\****************************************************/

// Go Text Protocol instance
pub(crate) struct GTP {
    board: Board,
}

/*****************************************************\
|****************    PRIVATE TYPES    ****************|
\*****************************************************/

// Enumerates all response types of the Go Text Protocol
// and handles sending them to the Protocol
enum GtpResponse {
    SUCCESS(String),
    ERROR(String),
    DEBUG(String, String), // response to protocol, debug message
}

// Enumerates all command types accepted from the Go Text Protocol
#[derive(EnumIter)]
enum GtpCommands {
    PROTOCOL_VERSION,
    NAME,
    VERSION,
    KNOWN_COMMAND,
    LIST_COMMANDS,
    QUIT,
    BOARDSIZE,
    CLEAR_BOARD,
    KOMI,
    PLAY,
    GENMOVE,
    SHOWBOARD,
}

/****************************************************\
|****************       HELPER       ****************|
\****************************************************/

impl fmt::Display for GtpCommands {
    // Implements to_string() for GtpCommands
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use GtpCommands::*;
        write!(
            f,
            "{}",
            match self {
                PROTOCOL_VERSION => "protocol_version",
                NAME => "name",
                VERSION => "version",
                KNOWN_COMMAND => "known_command",
                LIST_COMMANDS => "list_commands",
                QUIT => "quit",
                BOARDSIZE => "boardsize",
                CLEAR_BOARD => "clear_board",
                KOMI => "komi",
                PLAY => "play",
                GENMOVE => "genmove",
                SHOWBOARD => "showboard",
            }
        )
    }
}

impl GtpCommands {
    // Converts the given &str to its associated GtpCommand
    // Returned as an Option in case an invalid string is given
    fn from_string(command: &str) -> Option<GtpCommands> {
        use GtpCommands::*;
        match command {
            "protocol_version" => Some(PROTOCOL_VERSION),
            "name" => Some(NAME),
            "version" => Some(VERSION),
            "known_command" => Some(KNOWN_COMMAND),
            "list_commands" => Some(LIST_COMMANDS),
            "quit" => Some(QUIT),
            "boardsize" => Some(BOARDSIZE),
            "clear_board" => Some(CLEAR_BOARD),
            "komi" => Some(KOMI),
            "play" => Some(PLAY),
            "genmove" => Some(GENMOVE),
            "showboard" => Some(SHOWBOARD),
            _ => None,
        }
    }
}

/*****************************************************\
|****************         GTP         ****************|
\*****************************************************/

impl GtpResponse {
    // Writes the result of this GtpResponse to the Go Text Protocol
    fn write_to_gtp(self) {
        match self {
            GtpResponse::SUCCESS(result) => print!("= {}", Self::format_gtp_string(result)),
            GtpResponse::ERROR(result) => print!("? {}", Self::format_gtp_string(result)),
            GtpResponse::DEBUG(protocol_message, debug_message) => {
                eprint!("{}", Self::format_gtp_string(debug_message));
                print!("= {}", Self::format_gtp_string(protocol_message));
            },
        }
    }

    // Formats the given String such that it conforms to the Go Text Protocol
    fn format_gtp_string(input: String) -> String {
        if input.ends_with("\n\n") {
            input
        } else {
            Self::format_gtp_string(format!("{input}\n"))
        }
    }
}

impl GTP {
    // Creates a new instance of the Go Text Protocol
    pub(crate) fn new() -> GTP {
        GTP {
            board: Board::new(BoardSize::NINETEEN),
        }
    }

    // Starts a Go Text Protocol listener for the Go Engine
    pub(crate) fn start(mut self) -> io::Result<()> {
        use std::io;
        let mut buffer = String::new();
        loop {
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            let arguments: Vec<&str> = buffer.trim().split(" ").collect();
            if arguments.len() > 0 && !self.gtp_commands(&arguments) {
                break;
            }
        }

        Ok(())
    }

    // Handles input arguments given from the Go Text Protocol
    // and sends them to their respective command function
    // Returns true if the Protocol should remain open, else false.
    fn gtp_commands(&mut self, args: &[&str]) -> bool {
        use GtpCommands::*;
        if let Some(command) = GtpCommands::from_string(args[0]) {
            let response: GtpResponse = match command {
                PROTOCOL_VERSION => self.protocol_version(),
                NAME => self.name(),
                VERSION => self.version(),
                KNOWN_COMMAND => self.known_command(&args[1..]),
                LIST_COMMANDS => self.list_commands(),
                QUIT => return false,
                BOARDSIZE => self.boardsize(&args[1..]),
                CLEAR_BOARD => self.clear_board(),
                KOMI => self.komi(&args[1..]),
                PLAY => self.play(&args[1..]),
                GENMOVE => self.genmove(&args[1..]),
                SHOWBOARD => self.showboard(),
            };
            response.write_to_gtp();
        }

        true
    }

    // Returns the Go Text Protocol version this program conforms to
    fn protocol_version(&self) -> GtpResponse {
        GtpResponse::SUCCESS("2".to_string())
    }

    // Returns the name of this Go Engine
    fn name(&self) -> GtpResponse {
        GtpResponse::SUCCESS("TBD".to_string())
    }

    // Returns the version of this Go Engine
    fn version(&self) -> GtpResponse {
        GtpResponse::SUCCESS("0".to_string())
    }

    // args[0] = command name to check
    // Checks if the given command name is a command this engine supports
    // Gives a GtpResponse containing true if the command is known, false otherwise
    fn known_command(&self, args: &[&str]) -> GtpResponse {
        if args.len() < 1 {
            GtpResponse::ERROR("No command argument given".to_string())
        } else {
            GtpResponse::SUCCESS(GtpCommands::from_string(args[0]).is_some().to_string())
        }
    }

    // Lists all commands supported by this Go Engine
    fn list_commands(&self) -> GtpResponse {
        let mut command_list = String::from("");
        for command_name in GtpCommands::iter() {
            command_list = format!("{command_list}{command_name}\n");
        }
        GtpResponse::SUCCESS(command_list)
    }

    // args[0] = new board size
    // If given a valid BoardSize, clears the current board
    // and sets its board size to the given size
    // Returns an empty response unless an error occurs
    fn boardsize(&mut self, args: &[&str]) -> GtpResponse {
        if args.len() > 0 {
            if let Ok(num) = args[0].parse::<u16>() {
                if let Some(size) = BoardSize::from_u16(num) {
                    self.board = Board::new(size);
                    GtpResponse::SUCCESS(String::new())
                } else {
                    GtpResponse::ERROR(format!("Invalid size given to boardsize: {num}"))
                }
            } else {
                GtpResponse::ERROR(format!(
                    "Non-numeric size argument given to boardsize: {}",
                    args[0]
                ))
            }
        } else {
            GtpResponse::ERROR("No size argument given to boardsize".to_string())
        }
    }

    // Resets the board to an empty state
    // Returns an empty response
    fn clear_board(&mut self) -> GtpResponse {
        self.board = Board::new(self.board.size);
        GtpResponse::SUCCESS(String::new())
    }

    // args[0] = new decimal komi value 
    // Sets the komi of the current game to the given value
    // Returns an empty response unless an error occurs
    fn komi(&mut self, args: &[&str]) -> GtpResponse {
        if args.len() < 1 {
            return GtpResponse::ERROR("Komi value argument not given to command".to_string());
        }

        let komi_value = args[0].parse::<f64>();
        if komi_value.is_err() {
            return GtpResponse::ERROR(format!("Invalid komi argument given: {}", args[0]));
        }

        self.board.komi = komi_value.unwrap();
        GtpResponse::SUCCESS(String::new())
    }

    // args[0] = Color ("B", "W"), args[1] = intersection to play at in Go Notation (ex. "Q16")
    // Attempts to play a stone for the given color at the given intersection
    // If successful, returns an empty successful response
    // Else, returns an error response "Invalid move"
    fn play(&mut self, args: &[&str]) -> GtpResponse {
        if args.len() < 2 {
            return GtpResponse::ERROR("Not enough arguments given to play command".to_string());
        }

        let color = Color::from_string(args[0]);
        let intersection = Intersection::from_string(args[1]);

        if color.is_none() || intersection.is_none() {
            return GtpResponse::ERROR("syntax error".to_string()); // GTP required error message
        }

        if !self.board.play(Move::MOVE(intersection.unwrap(), color.unwrap())) {
            return GtpResponse::ERROR("invalid move".to_string()); // GTP required error message
        }

        GtpResponse::SUCCESS(String::new())
    }

    // TODO: IMPLEMENT
    // args[0] = Color ("B", "W")
    // Attempts to generate an engine move for the given color in the current Board position
    // Outputs the intersection to play at in Go Notation, "pass" if the engine wishes to pass,
    // or "resign" if the engine is resigning
    fn genmove(&mut self, args: &[&str]) -> GtpResponse {
        if args.len() < 1 {
            return GtpResponse::ERROR("Not enough arguments given to genmvove command".to_string());
        }
        
        let mov = match args[0] {
            "B" => generate_move(&self.board, Color::BLACK, 30),
            "W" => generate_move(&self.board, Color::WHITE, 30),
            _ => return GtpResponse::ERROR("Invalid color given to genmove".to_string()),
        };
        
        match mov {
            Move::MOVE(intsc, _) => {
                self.board.play(mov);
                GtpResponse::SUCCESS(intsc.to_string())
            },
            Move::PASS => GtpResponse::SUCCESS("pass".to_string()),
        }
    }

    // Returns a successful GtpResponse containing a rendering of the current Board position
    fn showboard(&self) -> GtpResponse {
        GtpResponse::SUCCESS(self.board.to_string())
    }
}
