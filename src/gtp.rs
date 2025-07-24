use std::io;
use super::*;
use board::*;

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
    DEBUG(String),
}

/*****************************************************\
|****************         GTP         ****************|
\*****************************************************/

impl GtpResponse {
    // Writes the result of this GtpResponse to the Go Text Protocol
    fn write_to_gtp(self) {
        match self {
            GtpResponse::SUCCESS(result) => println!("= {result}"),
            GtpResponse::ERROR(result) => println!("? {result}"),
            GtpResponse::DEBUG(result) => eprintln!("{result}"),
        }
    }
}

impl GTP {
    // Starts a Go Text Protocol listener for the Go Engine
    pub(crate) fn start() -> io::Result<()> {
        use std::io;
        let mut buffer = String::new();
        loop {
            buffer.clear();
            io::stdin().read_line(&mut buffer)?;
            if buffer.eq_ignore_ascii_case("quit\r\n") {
                break;
            }
            println!("{}", buffer);
        }

        Ok(())
    }
}