use serde::ser;

#[derive(Debug)]
pub struct CoronaError(String);

impl std::fmt::Display for CoronaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { todo!() }
}

impl std::error::Error for CoronaError {}

impl ser::Error for CoronaError {
    fn custom<T>(msg:T)->Self where T: std::fmt::Display {
        Self(msg.to_string())
    }
}