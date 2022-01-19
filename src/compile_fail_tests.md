# The following snippets fail to compile

```rust ,compile_fail
let x: u32 = match <u32 as TryFrom<u8>>::try_from(42) {
    | Ok(it) => it,
    | Err(unreachable) => unreachable, // Error, expected `u32`, found `Infallible`
};
```

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->
