use crate::ser::SerResult;
use serde::Serialize;

pub mod error;
pub mod ser;

pub fn write(value: impl Serialize, target: impl std::io::Write) -> SerResult {
    value.serialize(&mut ser::Corona::new(target))
}

pub fn to_file(value: impl Serialize, target: impl AsRef<std::path::Path>) -> SerResult {
    value.serialize(&mut ser::Corona::new(std::fs::File::create(target)?))
}

pub fn to_string(value: impl Serialize) -> Result<String, error::CoronaError> {
    let mut out = Vec::new();
    value.serialize(&mut ser::Corona::new(&mut out))?;
    Ok(String::from_utf8(out)?)
}