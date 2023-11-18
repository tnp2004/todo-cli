use crate::config::Cfg;
use crate::comd::{self, Comd};
use crate::Result;

pub struct Todo {
    pub header_fields: Vec<String>,
    pub config: Cfg,
}

impl Todo {
    pub fn init(cfg: Cfg) -> Self {
        Self {
            config: cfg,
            header_fields: vec![
                "id".to_string(),
                "task".to_string(),
                "status".to_string(),
                "created_at".to_string(),
                "updated_at".to_string(),
            ],
        }
    }

    pub fn run(&self) -> Result<()> {
        let command = comd::read();
        command.match_command(&self)?;
        
        Ok(())
    }
}
