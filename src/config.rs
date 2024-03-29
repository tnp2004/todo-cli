use crate::Result;
use config::{builder::DefaultState, Config, ConfigBuilder};
use std::collections::HashMap;

pub trait TConfig {
    fn set_path(&mut self, path: String) -> Result<()>;
    fn get_path(&self) -> &String;
}

pub struct Cfg {
    filename: String,
    builder: ConfigBuilder<DefaultState>,
    cfg: HashMap<String, String>,
}

impl Cfg {
    pub fn init(filename: String) -> Self {
        Self {
            filename,
            builder: Config::builder(),
            cfg: HashMap::new(),
        }
    }

    pub fn load_config(&mut self) -> Result<()> {
        let config = self
            .builder.clone()
            .add_source(config::File::with_name(&self.filename))
            .add_source(config::Environment::with_prefix("APP"));

        self.builder = config.clone();
        self.cfg = config.build().unwrap().try_deserialize::<HashMap<String, String>>().unwrap();

        Ok(())
    }
}

impl TConfig for Cfg {
    fn set_path(&mut self, path_value: String) -> Result<()> {
        self.builder = self.builder.clone().set_override("path", path_value).unwrap();
    //    self.cfg.insert("path".to_string(), path_value);

        Ok(())
    }

    fn get_path(&self) -> &String {
        self.cfg.get("path").unwrap()
    }

}