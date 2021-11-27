use serde::ser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoronaError {
    #[error("IO error while writing code: {0}")]
    Io(#[from] std::io::Error),
    #[error("Unknown error: {0}")]
    Custom(String),
}

impl ser::Error for CoronaError {
    fn custom<T>(msg:T)->Self where T: std::fmt::Display {
        Self::Custom(msg.to_string())
    }
}