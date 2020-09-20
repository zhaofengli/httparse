#[cfg(not(all(
    httparse_simd,
    any(
        target_arch = "x86",
        target_arch = "x86_64",
    ),
)))]
mod fallback;

#[cfg(not(all(
    httparse_simd,
    any(
        target_arch = "x86",
        target_arch = "x86_64",
    ),
)))]
pub use self::fallback::*;
