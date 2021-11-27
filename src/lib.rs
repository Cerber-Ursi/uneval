//! Serde serializer generating Rust code.
//!
//! This crate can be used to "embed" something into code, having only some serialized
//! data, like JSON or YAML. This way, you'll mostly escape runtime cost of deserialization,
//! nearly as if you've written the same data directly in code by hand.
//! Of course, in most cases this cost is already negligible, but for crates which use
//! large blobs of data this crate can come in handy, improving startup times.
//!
//! ## Usage
//! In general, to embed some code into crate, you have to use the build script
//! and [`include!`][include] macro. Inside the build script, you'll generate
//! some code with one of the [functions][funcs] provided by `uneval`,
//! and then include the generated file, like this:
//! ```ignore
//! let value = include!(concat!(env!(OUT_DIR), "/file_name.rs"));
//! ```
//!
//! ## Limitations
//! There are some cases when `uneval` will be unable to generate valid code. Namely:
//! 1. Since Serde doesn't provide us the full path to the type in question (and in most cases it's simply unable to),
//! all the structs and enums used during value construction must be in scope.
//! As a consequence, all of them must have distinct names - otherwise, there will be name clashes.
//! 2. This serializer is intended for use with derived implementation. It may return bogus results
//! when used with customized `Serialize`.
//! 3. It is impossible to consume code for the type with private fields outside from the module it is defined in.
//! In fact, to be able to use this type with `uneval`, you'll have to distribute two copies of your crate,
//! one of which would only export the definition with derived `Serialize` to be used by serializer
//! during the build-time of the second copy. (Isn't this a bit too complex?)
//! 4. It is impossible to use empty tuple structs (i.e. `Empty()`).
//! From the Serde's point of view, they are indistinguishable from unit structs (i.e. `Unit`),
//! but the same Rust syntax can't be used for both, and, since ordinary unit structs are much
//! more common, it was decided to correctly handle them.
//!
//! [include]: https://doc.rust-lang.org/stable/std/macro.include.html

pub mod error;
pub mod ser;
pub mod funcs;
pub mod helpers;

pub use funcs::{to_file, to_out_dir, to_string, write};