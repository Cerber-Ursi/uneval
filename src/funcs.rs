//! Convenience functions to be used with Uneval.

use crate::ser::{SerResult, Uneval};
use crate::error::UnevalError;
use serde::Serialize;

/// Write generated Rust code to the provided [`Write`][std::io::Write] implementation.
pub fn write(value: impl Serialize, target: impl std::io::Write) -> SerResult {
    value.serialize(&mut Uneval::new(target))
}

/// Writes generated Rust code to file.
///
/// This is probably the most common way to use `uneval`. When Cargo runs your crate's build task,
/// it sets the `OUT_DIR` environment variable to the path to build target directory (see
/// [Cargo reference](https://doc.rust-lang.org/cargo/reference/environment-variables.html) for more).
/// So, you can use it in two steps:
/// 1. Generate the Rust code and write it to temporary file:
/// ```no_run
/// # let value = ();
/// let path: std::path::PathBuf = [
///     std::env::var("OUT_DIR").expect("No build target path set"),
///     "file_name.rs".into()
/// ].iter().collect();
/// uneval::to_file(value, path).expect("Write failed");
/// ```
/// 2. [Include][include] the generated Rust code wherever it is needed:
/// ```ignore
/// let value = include!(concat!(env!(OUT_DIR), "/file_name.rs"));
/// ```
///
/// [include]: https://doc.rust-lang.org/stable/std/macro.include.html
pub fn to_file(value: impl Serialize, target: impl AsRef<std::path::Path>) -> SerResult {
    value.serialize(&mut Uneval::new(std::fs::File::create(target)?))
}

/// Convenience wrapper around [`to_file`].
/// 
/// This function finds out where the output directory is by looking at `OUT_DIR` environment variable
/// and creates the file with the provided name there.
pub fn to_out_dir(value: impl Serialize, file_name: impl AsRef<str>) -> SerResult {
    let path: std::path::PathBuf = [
        std::env::var("OUT_DIR").expect("OUT_DIR not set, check if you're running this from the build script"),
        file_name.as_ref().into()
    ].iter().collect();
    value.serialize(&mut Uneval::new(std::fs::File::create(path)?))
}

/// Obtain string with generated Rust code.
pub fn to_string(value: impl Serialize) -> Result<String, UnevalError> {
    let mut out = Vec::new();
    value.serialize(&mut Uneval::new(&mut out))?;
    Ok(String::from_utf8(out)?)
}