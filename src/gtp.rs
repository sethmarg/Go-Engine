use super::*;
use board::*;
use std::fmt::Formatter;
use std::{fmt, io};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
/****************************************************\
|****************    GLOBAL TYPES    ****************|
\****************************************************/

pub(crate) struct GTP {
    board: Board,
}

/*****************************************************\
|****************    PRIVATE TYPES    ****************|
\*****************************************************/

enum GtpResponse {
    SUCCESS(String),
    ERROR(String),
    DEBUG(String, String), // response to protocol, debug message
}

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

    fn protocol_version(&self) -> GtpResponse {
        GtpResponse::SUCCESS("2".to_string())
    }

    fn name(&self) -> GtpResponse {
        GtpResponse::SUCCESS("TBD".to_string())
    }

    fn version(&self) -> GtpResponse {
        GtpResponse::SUCCESS("0".to_string())
    }
    fn known_command(&self, args: &[&str]) -> GtpResponse {
        if args.len() < 1 {
            GtpResponse::ERROR("No command argument given".to_string())
        } else {
            GtpResponse::SUCCESS(GtpCommands::from_string(args[0]).is_some().to_string())
        }
    }

    fn list_commands(&self) -> GtpResponse {
        let mut command_list = String::from("");
        for command_name in GtpCommands::iter() {
            command_list = format!("{command_list}{command_name}\n");
        }
        GtpResponse::SUCCESS(command_list)
    }

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

    fn clear_board(&mut self) -> GtpResponse {
        self.board = Board::new(self.board.size);
        GtpResponse::SUCCESS(String::new())
    }

    fn komi(&mut self, args: &[&str]) -> GtpResponse {
        GtpResponse::DEBUG("".to_string(), "to be implemented".to_string())
    }

    fn play(&mut self, args: &[&str]) -> GtpResponse {
        GtpResponse::DEBUG("".to_string(), "to be implemented".to_string())
    }

    fn genmove(&self, args: &[&str]) -> GtpResponse {
        GtpResponse::DEBUG("".to_string(), "to be implemented".to_string())
    }
    
    fn showboard(&self) -> GtpResponse {
        GtpResponse::SUCCESS(self.board.render())
    }
}
