use crate::error::Error;

pub trait Action {
    fn add(&self, task: &String, path: &String) -> Result<(), Error>;
    fn remove(&self, number: i32, path: &String) -> Result<(), Error>;
    fn show(&self, path: &String) -> Result<(), Error>;
    fn update_status(&self, number: i32, status: &String, path: &String) -> Result<(), Error>;
    fn export(&self, path: &String, export_path: &String) -> Result<(), Error>;
}

pub trait ArgParser {
    fn parse_arg(&self, arg_name: &String) -> Option<&String>;
    fn parse_sub_arg(&self, action: &String, arg_name: &String) -> Option<&String>;
}