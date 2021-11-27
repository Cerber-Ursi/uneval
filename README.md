# corona

Makes Serde serialize your data to the Rust source code.

### Why?

This crate was inspired by the [Stack Overflow question](https://stackoverflow.com/questions/58359340/deserialize-file-using-serde-json-at-compile-time). In short, if you want to make some data in the human-readable format, such as JSON, to be deserialized at compile-time and build into the binary, as if it was written by hand, then this crate can possibly help you.

### How to?

This crate is intended to be used from the build script. It will serialize anything you provide to it to any path you provide (or to the arbitrary [`io::Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html) implementation, if you want to). Then, you'll [`include!`](https://doc.rust-lang.org/stable/std/macro.include.html) the generated file wherever you want to use it.

### Is it really that simple?

Well... not. There are several limitations.

1. All the types used in the serialized struct must be in scope on the include site. Serde doesn't provide the qualified name (i.e. path) to the serializer, only the direct name. The probably easiest way is to use the serialized data as following:
```rust
let value = {
    use ::path::to::Type1;
    // ...and other types
    include!("path/to/file.rs")
}
```
or the similar construction using [`lazy_static`](http://crates.io/crates/lazy_static).
2. As a consequence, all the types used by the serialized one must have distinct names (or they'll clash with each other).
3. Deserializer isn't implemented. This is intentional, since this crate isn't really intended for runtime usage.
4. This serializer is intended to use with derived implementation. It may return bogus results when used with customized `Serialize`.
5. It is impossible to serialize the struct with private fields outside from the module it is defined in. In fact, to be able to serialize this type at all, you'll have to distribute two copies of your crate, one of which would only export the definition with derived `Serialize` to be used by this crate during the build-time of the second copy. (Isn't this a bit too complex?)