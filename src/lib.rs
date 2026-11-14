//! # charry
//!
//! This crate provides [`CharIn`], a buffered UTF-8 decoder that wraps any
//! [`io::Read`] byte source and yields [`char`]s. It works in both `std`
//! and `no_std` (with `alloc`) environments.
//!
//! Invalid UTF-8 sequences are surfaced as [`Error::InvalidUtf8`] together
//! with the byte offset at which the invalid sequence begins, and the reader
//! advances past the offending byte so subsequent reads can continue.
//!
//! ## Features
//!
//! - `std` *(enabled by default)* — provides a blanket [`io::Read`]
//!   implementation for all [`std::io::Read`] types.
//!
//! ## Example
//!
//! ```rust
//! use charry::CharIn;
//!
//! let mut reader = CharIn::new(&b"hello"[..]);
//! while let Some(c) = reader.next().unwrap() {
//!     print!("{c}");
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs, missing_debug_implementations, clippy::pedantic)]
#![forbid(unsafe_code)]

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

extern crate alloc;

mod buf;
mod bytes;
mod dbg;
mod error;
mod new;
mod unicode;
mod utf;

pub mod io;

use alloc::boxed::Box;
use utf::{DecodeStep, decode_char};

pub use error::Error;

type Result<T, E> = core::result::Result<T, crate::Error<E>>;

/// A buffered UTF-8 character reader.
///
/// `CharIn` wraps a byte-oriented [`io::Read`] backend and decodes the
/// underlying byte stream into [`char`]s. It maintains an internal buffer
/// so that multi-byte UTF-8 sequences can be assembled even when the
/// backend returns them in fragments.
///
/// Use [`CharIn::next`] to consume a character, or [`CharIn::peek`] to look
/// ahead without consuming.
pub struct CharIn<R: io::Read + Unpin> {
    inner: R,
    buf: Box<[u8]>,
    pos: usize,
    end: usize,
    eof: bool,
    base_offset: u64,
}

impl<R: io::Read + Unpin> CharIn<R> {
    /// The default internal buffer capacity (8 KiB).
    const DEFAULT_CAPACITY: usize = 8192;

    /// Returns the byte offset of the next character to be read.
    ///
    /// The offset counts bytes consumed from the start of the stream,
    /// including bytes belonging to already-consumed invalid UTF-8 sequences.
    #[inline]
    pub fn offset(&self) -> u64 {
        self.base_offset + self.pos as u64
    }
}
