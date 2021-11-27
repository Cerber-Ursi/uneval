//! Serde serializer generating Rust code.
//!
//! This crate can be used to "embed" something into code, having only some serialized
//! data, like JSON or YAML. This way, you'll mostly escape runtime cost of deserialization,
//! nearly as if you've written the same data directly in code by hand.
//! Of course, in most cases this cost is already negligible, but for crates which use
//! large blobs of data this crate can come in handy, improving startup times, and can
//! eliminate the need for `serde` as runtime dependency.
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
//! ## How does it work?
//!
//! Of course, we can't always directly construct the code for the desired value (more on this
//! in the [Limitations](#limitations) section below).
//! However, in many cases the information provided by Serde is enough.
//!
//! For every case, we'll provide an example of how the generated code can look like, as a sequence of
//! `let` statements, where the left part is written by hand and the right one is assumed to be generated.
//!
//! ### Primitives
//!
//! Number literals, such as `i8` or `f32`, are directly written into the output. The only tricky part
//! is that we have to use suffixed literals, e.g. `1u8` or `1.1f64` - otherwise we'd run into the problem
//! with the float values which are in fact integers, since they would be output as integer literals,
//! not as float ones (i.e. `1` and not `1.0`) and so wouldn't typecheck.
//!
//! Boolean and character literals are also simply written directly - no surprises here.
//!
//! Example:
//! ```
//! let _: i8 = 12i8;
//! let _: u128 = 12345u128;
//! let _: f32 = -1f32;
//! let _: f64 = 12345.6789f64;
//! let _: char = 'c';
//! let _: bool = true;
//! ```
//!
//! ### Strings
//! When Serde gives us something string-like, we have to make some kind of conversion, since
//! string literals are of type `&'static str`, and string-like fields in serializable structs are
//! usually of some owned type, like `String`. We assume that every such type would be convertible to
//! `String` using [`Into`][std::convert::Into], so we simply emit a string literal with call to `into`.
//!
//! Example:
//! ```
//! let _: String = "string value".into();
//! ```
//!
//! Byte strings are handled as byte sequences, [as recommended by Serde itself][::serde::Serializer::serialize_bytes],
//! and so we'll discuss them [below](#vec-like-types-sequences).
//!
//! ### Tuple structs and unit values
//!
//! Unit type (`()`), unit structs and unit variants (including `None`) are emitted simply by using
//! the type name. Tuple structs and variants (and newtype-flavored ones, including `Some`)
//! are emitted by writing  their name (with the enum name, if necessary), parenthesis,
//! and serializing the inner values.
//!
//! Example:
//! ```
//! struct TupleStruct((), Option<u8>, Option<u8>);
//! let _: TupleStruct = TupleStruct((), None, Some(1u8));
//! ```
//!
//! ### Vec-like types (sequences)
//!
//! `Vec`-like structures are constructed using the temporary `Vec`. We assume that every such type will
//! implement [`FromIterator`][std::iter::FromIterator], so we emit the call to `vec!` macro,
//! serialize the data and finalize the emit with call to `into_iter().collect()`.
//! This is not exactly zero-cost, but it seems that this is the minimal.
//!
//! Example:
//! ```
//! let _: Vec<u32> = vec![1u32, 2u32, 3u32].into_iter().collect();
//! ```
//!
//! ### Tuples and arrays
//!
//! That's where it becomes tricky.
//!
//! The problem is that Serde doesn't distinguish between this two kinds of values: they both are treated
//! as sequences with known length, called "tuples" internally; as a consequence, we don't know at the emit time,
//! which of them we'll be generating. But in the Rust code, they are created with entirely different syntax,
//! and there's no easy way to convert one into another. So, we decided to emit a little "runtime"
//! (consisting of small `#[inline]` functions, so it should in fact be zero-cost), which will
//! correctly handle the data according to the type being requested.
//!
//! The idea is, in fact, directly borrowed from the [`collect`]/[`FromIterator`] pair: we can call `collect`
//! on every iterator value, and, as long as the target type implements `FromIterator` with the necessary
//! parameters, `collect` will do its job. We're using not the trait method, but the free function (the reason is
//! that with the trait we would sometimes have a chain of type inferences, which Rust is unable to solve);
//! however, this doesn't change the overall picture.
//!
//! [`collect`]: std::iter::Iterator::collect
//! [`FromIterator`]: std::iter::FromIterator
//!
//! In general, here's what being generated:
//! - A `FromTuple<T>` trait with `from_tuple(input: T) -> Self` associated function.
//! - Two implementations: `impl<T> FromTuple<(T,...,T,)> for [T; N]` and
//! `impl<T1, ... TN> FromTuple<(T1,...TN,)> for (T1,...TN,)`.
//! - Function `convert<T1, ... TN, Out: FromTuple<(T1,...TN,)>>(tuple: (T1,...TN,)) -> Out`,
//! which simply calls `Out::from_tuple(tuple)`.
//!
//! Then, the value itself is created by the call to `convert`, with tuple of serialized values as argument.
//! Depending on whether the target expects the array or tuple, `convert` will select one particular implementation.
//!
//! Example:
//! ```
//! let tuple: (i32, f32, String) = {
//!     trait FromTuple<T>: Sized {
//!         fn from_tuple(tuple: T) -> Self;
//!     }
//!
//!     impl<T> FromTuple<(T,T,T,)> for [T; 3] {
//!         #[inline]
//!         fn from_tuple(tuple: (T,T,T,)) -> Self {
//!             [tuple.0,tuple.1,tuple.2]
//!         }
//!     }
//!
//!     impl<T0,T1,T2> FromTuple<(T0,T1,T2,)> for (T0,T1,T2,) {
//!         #[inline]
//!         fn from_tuple(tuple: (T0,T1,T2,)) -> Self {
//!             tuple
//!         }
//!     }
//!
//!     #[inline]
//!     fn convert<T0,T1,T2, Out: FromTuple<(T0,T1,T2,)>>(tuple: (T0,T1,T2,)) -> Out {
//!         Out::from_tuple(tuple)
//!     }
//!
//!     convert((1i32,1f32,"tuple entry".into()))
//! };
//! // Check that the tuple is indeed created as desired.
//! assert_eq!(tuple, (1i32,1f32,"tuple entry".to_string()));
//!
//! let arr: [i32; 4] = {
//!     trait FromTuple<T>: Sized {
//!         fn from_tuple(tuple: T) -> Self;
//!     }
//!
//!     impl<T> FromTuple<(T,T,T,T,)> for [T; 4] {
//!         #[inline]
//!         fn from_tuple(tuple: (T,T,T,T,)) -> Self {
//!             [tuple.0,tuple.1,tuple.2,tuple.3]
//!         }
//!     }
//!
//!     impl<T0,T1,T2,T3> FromTuple<(T0,T1,T2,T3,)> for (T0,T1,T2,T3,) {
//!         #[inline]
//!         fn from_tuple(tuple: (T0,T1,T2,T3,)) -> Self {
//!             tuple
//!         }
//!     }
//!
//!     #[inline]
//!     fn convert<T0,T1,T2,T3, Out: FromTuple<(T0,T1,T2,T3,)>>(tuple: (T0,T1,T2,T3,)) -> Out {
//!         Out::from_tuple(tuple)
//!     }
//!
//!     convert((1,2,3,4))
//! };
//! // Check that the array is indeed created as desired.
//! assert_eq!(arr, [1, 2, 3, 4]);
//! ```
//!
//! ### Maps
//!
//! Since Rust doesn't have the notion of map literals, we can't construct one directly. However, standard map-like
//! types ([`HashMap`], [`BTreeMap`]) implement `FromIterator<(K, V)>`, i.e. they can be built from the iterator of
//! key-value pairs. `uneval` generates code according to this convention: we create a `Vec` of pairs, which is then
//! converted into map with `into_iter().collect()`.
//!
//! Example:
//! ```
//! let _: std::collections::HashMap<i32, String> = vec![
//!     (1, "first".into()),
//!     (100, "one hundredth".into()),
//! ].into_iter().collect();
//! ```
//!
//! [`HashMap`]: std::collections::HashMap
//! [`BTreeMap`]: std::collections::BTreeMap
//!
//! ### Structs
//!
//! Last but not the least, this case is relatively simple. Emitted code is simply the struct construction -
//! i.e. the struct name, the curly braces and a list of pairs of the form `{field name}: {serialized value}`.
//!
//! Example:
//! ```
//! struct Struct { boolean: bool, number: i32, string: String }
//! let _: Struct = Struct {
//!     boolean: true,
//!     number: 1i32,
//!     string: "string".into()
//! };
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

mod helpers;

pub mod error;
pub mod funcs;
pub mod ser;

pub use funcs::{to_file, to_out_dir, to_string, write};
