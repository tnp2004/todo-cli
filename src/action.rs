use crate::error::Error;

pub trait Action {
    fn add(&self, task: String) -> Result<(), Error>;
    fn remove(&self, task: String) -> Result<(), Error>;
}