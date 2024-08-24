use std::{env, sync::OnceLock};

use crate::{CustomError, Result};

pub fn config() -> &'static Config {
    // `static` its globally, here it only accessed in this function
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL - WHILE LOADING CONF - CAUSE: {ex:?}");
        })
    })
}

#[allow(non_snake_case)]
pub struct Config {
    // -- Web
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            // -- Web
            WEB_FOLDER: get_env("SERVER_WEB_FOLDER")?,
        })
    }
}

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| CustomError::ConfigMissingEnv(name))
}
