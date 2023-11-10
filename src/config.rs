use crate::Result;
use config::{builder::DefaultState, Config, ConfigBuilder};
use std::collections::HashMap;

pub trait TConfig {
    fn set_path(&self, path: String) -> Result<()>;
    fn get_path(&self) -> &String;
}

pub struct Cfg {
    path: String,
    builder: ConfigBuilder<DefaultState>,
}

impl Cfg {
    pub fn init() -> Self {
        Self {
            path: String::new(),
            builder: Config::builder(),
        }
    }
}

impl TConfig for Cfg {
    fn set_path(&self, path_value: String) -> Result<()> {
        Config::builder()
            .set_override("path", path_value)
            .unwrap()
            .add_source(config::File::with_name("./Settings"))
            .build()
            .unwrap();

        Ok(())
    }

    fn get_path(&self) -> &String {
        &self.path
    }
}

pub fn config() {
    let mut settings = Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("./Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap)
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    // let settings = Config::builder()
    //     .set_override("name", "lol")
    //     .unwrap()
    //     // Add in `./Settings.toml`
    //     .add_source(config::File::with_name("./Settings"))
    //     .add_source(config::Environment::with_prefix("APP"))
    //     .build()
    //     .unwrap();

    // println!(
    //     "{:?}",
    //     settings
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap()
    //         .get("name")
    // );
}
