pub trait Action {
    fn add(&self, task: String) -> Result<(), Box<csv::Error>>;
    fn remove(&self, task: String) -> Result<(), Box<csv::Error>>;
}