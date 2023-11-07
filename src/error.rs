use std::convert::From;
use std::fmt::Display;
use std::io;

pub enum Error {
    TaskNotFound,
    CommandNotFound,
    CsvError(String),
    IO(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let message = match self {
            Self::TaskNotFound => "Task not found",
            Self::CommandNotFound => "Command not found",
            Self::CsvError(e) => e,
            Self::IO(e) => e,
        };

        write!(f , "{}", message)
    }
}

impl From<csv::Error> for Error {
    fn from(e: csv::Error) -> Self {
        Self::CsvError(e.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::CsvError(e.to_string())
    }
}