use std::env::VarError;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("{name} is not set")]
pub struct EnvNotSet {
    name: String,
}

impl From<(&str, VarError)> for EnvNotSet {
    fn from((name, _): (&str, VarError)) -> Self {
        Self { name: name.to_string() }
    }
}

#[derive(Debug, Clone)]
pub struct Env {
    pub kurrentdb_url: String,
    pub database_url: String,
}

impl Env {
    pub fn load() -> Result<Self, EnvNotSet> {
        Ok(Self {
            kurrentdb_url: required("KURRENTDB_URL")?,
            database_url: required("DATABASE_URL")?,
        })
    }
}

fn required(key: &str) -> Result<String, EnvNotSet> {
    std::env::var(key).map_err(|e| EnvNotSet::from((key, e)))
}
