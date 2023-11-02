pub trait Action {
    fn add(&self, task: String);
    fn remove(&self, task: String);
}