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
    fn write_to_gtp(self) {
        match self {
            GtpResponse::SUCCESS(result) => println!("= {result}"),
            GtpResponse::ERROR(result) => println!("? {result}"),
            GtpResponse::DEBUG(result) => eprintln!("{result}"),
        }
    }
}