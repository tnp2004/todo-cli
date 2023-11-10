use crate::error::Error;

pub trait Action {
    fn add(&self, task: &String, path: &String) -> Result<(), Error>;
    fn remove(&self, task: &String, path: &String) -> Result<(), Error>;
    fn show(&self, path: &String) -> Result<(), Error>;
}

pub trait ArgParser {
    fn parse_arg(&self, arg_name: &String) -> Result<&String, Error>;
    fn parse_sub_arg(&self, action: &String, arg_name: &String) -> Result<&String, Error>;
}