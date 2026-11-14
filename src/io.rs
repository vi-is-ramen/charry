//! Byte-oriented input traits.
//!
//! This module defines the [`Read`] trait — a minimal abstraction over a
//! source of bytes. When the `std` feature is enabled, [`Read`] is a
//! pass-through alias for [`std::io::Read`], so any `std::io::Read`
//! implementation can be used with this crate.
//!
//! When `core_io` stabilizes, this module will be replaced by
//! `core::io::Read` and the `std` gate will be removed.

/// A byte-oriented input source.
///
/// This trait mirrors [`std::io::Read`] but is available in `no_std`
/// environments (once a suitable blanket impl or concrete impl is provided).
pub trait Read {
    /// The error type returned by [`Read::read`].
    type Error;

    /// Reads up to `out.len()` bytes into `out`.
    ///
    /// Returns the number of bytes read. A return value of `0` indicates
    /// end of stream (or that `out` is empty).
    ///
    /// # Errors
    ///
    /// Returns `Self::Error` if the underlying source fails.
    fn read(&mut self, out: &mut [u8]) -> Result<usize, Self::Error>;
}

/// Blanket implementation: on `std`, any [`std::io::Read`] is a [`Read`].
///
/// This makes [`crate::CharIn`] usable with `std` I/O types such as files,
/// sockets, and `&[u8]` slices without any adapter.
#[cfg(feature = "std")]
impl<T> Read for T
where
    T: std::io::Read,
{
    type Error = std::io::Error;

    fn read(&mut self, out: &mut [u8]) -> Result<usize, Self::Error> {
        std::io::Read::read(self, out)
    }
}
