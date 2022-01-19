/*!
# `::never-say-never`

[![Repository](https://img.shields.io/badge/repository-GitHub-brightgreen.svg)](
https://github.com/danielhenrymantilla/never-say-never.rs)
[![Latest version](https://img.shields.io/crates/v/never-say-never.svg)](
https://crates.io/crates/never-say-never)
[![Documentation](https://docs.rs/never-say-never/badge.svg)](
https://docs.rs/never-say-never)
[![MSRV](https://img.shields.io/badge/MSRV-1.14.0-white)](
https://gist.github.com/danielhenrymantilla/8e5b721b3929084562f8f65668920c33)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](
https://github.com/rust-secure-code/safety-dance/)
[![License](https://img.shields.io/crates/l/never-say-never.svg)](
https://github.com/danielhenrymantilla/never-say-never.rs/blob/master/LICENSE-ZLIB)
[![CI](https://github.com/danielhenrymantilla/never-say-never.rs/workflows/CI/badge.svg)](
https://github.com/danielhenrymantilla/never-say-never.rs/actions)

<!-- Templated by `cargo-generate` using https://github.com/danielhenrymantilla/proc-macro-template -->

The `!` type. In stable Rust. Since `1.14.0`.

Better than an `enum Never {}` definition would be, since an instance of
type `!` automagically coerces to any type, whereas an instance of
`enum EmptyEnum {}` needs an explicit `match it {}`.

  - Currently, [`::core::convert::Infallible`] is a sad instance of the
    latter.

[`::core::convert::Infallible`]: https://doc.rust-lang.org/1.58.0/core/convert/enum.Infallible.html

That is, the following fails to compile:

```rust ,compile_fail
let x: u32 = match <u32 as TryFrom<u8>>::try_from(42) {
    | Ok(it) => it,
    | Err(unreachable) => unreachable, // Error, expected `u32`, found `Infallible`
};
```

but the following doesn't!

```rust
use ::never_say_never::Never;

let x: u32 = match Ok::<_, Never>(42) {
    | Ok(it) => it,
    | Err(unreachable) => unreachable,
};
```
*/

#![no_std]
#![forbid(unsafe_code)]

/// Workaround for `fn_traits` and/or `unboxed_closures`
mod fn_traits {
    pub
    trait FnOnce<Args> {
        type Output;
    }

    impl<F, R> FnOnce<()> for F
    where
        F : ::core::ops::FnOnce() -> R,
    {
        type Output = R;
    }
}

/// The `!` type. See [the main docs for more info][`crate`].
pub
type Never = <
    fn() -> !
    as
    fn_traits::FnOnce<()>
>::Output;
