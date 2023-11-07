pub enum Status {
    Holding,
    Implement,
    Finished,
    Cancelled,
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Holding => "holding".to_string(),
            Status::Implement => "implement".to_string(),
            Status::Finished => "finished".to_string(),
            Status::Cancelled => "cancelled".to_string(),
        }
    }
} 