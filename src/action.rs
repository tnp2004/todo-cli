use crate::error::Error;

pub trait Action {
    fn add(&self, task: &String, path: &String) -> Result<(), Error>;
    fn remove(&self, task: &String, path: &String) -> Result<(), Error>;
    fn show(&self, path: &String) -> Result<(),Error>;
}