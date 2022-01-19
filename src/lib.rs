#![cfg_attr(feature = "better-docs",
    cfg_attr(all(), doc = include_str!("../README.md")),
)]
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


#[cfg_attr(feature = "ui-tests",
    cfg_attr(all(), doc = include_str!("compile_fail_tests.md")),
)]
mod _compile_fail_tests {}
