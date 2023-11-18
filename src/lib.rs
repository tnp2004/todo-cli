pub mod todo;
pub mod action;
pub mod file;
pub mod error;
pub mod status;
pub mod config;
pub mod utils;
pub mod arg_parser;
pub mod comd;

pub use error::Error;
pub use utils::time;

pub type Result<T> = std::result::Result<T, Error>;