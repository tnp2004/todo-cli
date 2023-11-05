use crate::error::Error;

pub trait Action {
    fn add(&self, task: &String, path: &String) -> Result<(), Error>;
    fn remove(&self, task: &String, path: &String) -> Result<(), Error>;
    fn show(&self, path: &String) -> Result<(),Error>;
}

pub trait ValueParser {
    fn parse_arg(&self, action: &String, value: &String, path: Option<&String>) -> Result<(), Error>;
    fn parse_arg_flag(&self, action: &String, value: &String, path: Option<&String>) -> Result<(), Error>;
}