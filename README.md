# uneval

Makes [Serde](http://serde.rs) serialize your data to the Rust source code.

### Why?

This crate was inspired by the [Stack Overflow question](https://stackoverflow.com/questions/58359340/deserialize-file-using-serde-json-at-compile-time). In short, if you want to make some data in the human-readable format, such as JSON, to be deserialized at compile-time and build into the binary, as if it was written by hand, then this crate can possibly help you.

### How to?

This crate is intended to be used from the build script. It will serialize anything you provide to it to any path you provide (or to the arbitrary [`io::Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html) implementation, or into `String`, if you want to). Then, you'll [`include!`](https://doc.rust-lang.org/stable/std/macro.include.html) the generated file wherever you want to use it.

### How does it work?

See the [crate documentation](https://docs.rs/uneval) for details. In short, we use information provided by Serde to emit the code, which, when assigned to the variable of correct type, will provide all necessary conversions by using `Into` and iterators.

### Is it really that simple?

Well... not. There are several limitations.

1. All the types used in the serialized struct must be in scope on the include site. Serde doesn't provide the qualified name (i.e. path) to the serializer, only the "last" name. The probably easiest way is to use the serialized data as following:
```rust
let value: MainType = {
    use ::path::to::Type1;
    // ...and other types
    include!("path/to/file.rs")
}
```
or the similar construction using [`lazy_static`](http://crates.io/crates/lazy_static).

2. As a consequence, all the types used by the serialized one must have distinct names (or they'll clash with each other).
3. Deserializer isn't implemented. This is intentional, since this crate isn't really intended for runtime usage. Well, in fact, the deserializer *is* implemented - it's just the Rust compiler itself.
4. This serializer is intended for use with derived implementation. It may return bogus results when used with customized `Serialize`.
5. It is impossible to serialize the struct with private fields outside from the module it is defined in. In fact, to be able to serialize this type at all, you'll have to distribute two copies of your crate, one of which would only export the definition with derived `Serialize` to be used by this crate during the build-time of the second copy. (Isn't this a bit too complex?)
6. It is impossible to use empty tuple structs (i.e. `Empty()`). From the Serde's point of view, they are indistinguishable from unit structs (i.e. `Unit`), but the same Rust syntax can't be used for both, and, since ordinary unit structs are much more common, it was decided to abandon the empty tuples.

If you find any other case where this doesn't work, feel free to open an issue - we'll either fix the code or document the newly-found limitation.

### Testing

This crate uses [`batch_run`](https://crates.io/crates/batch_run) to run its tests.

The common structure of test cases is like following:
- File named `definition.rs` contains the necessary types.
- File named `{test_name}-main.rs` includes `definition.rs` as module. It contains the `main` function, which creates an instance of some type from `definition.rs`, generates the corresponding Rust code in `generated.rs` and launches `{test_name}-user.rs` through `batch_run`.
- File named `{test_name}-user.rs` includes `definition.rs` as module and `generated.rs` through call to `include!`. It checks that the generated code indeed creates the data equal to what was created initially.

Testing data itself is defined in [test_fixtures/data.toml], and is in the following format:
- Section name in TOML corresponds to the name of test case. Note that this is not the Cargo test, but the item in the `batch_run`'s batch.
- Field `main_type` corresponds to the type which serialization is being tested.
- If there are several types (for example, in the nested struct), all other types except for main one should be listed under `support_types` as a comma-separated list. These, together with the `main_type`, will be included in `{test_name}-user.rs` as imports.
- Field `definition` is literally copied into the `definition.rs`. It's necessary to derive `Debug`, `Serialize` and `PartialEq` on all the types there, since these traits are used during test entry run.
- Field `value` is literally copied in two places: first, the `{test_name}-main.rs`, where the code is generated; second, in `{test_name}-user.rs`, where test checks two values for equality.

# License

MIT