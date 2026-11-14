/// Errors that can occur while reading characters with [`CharIn`].
#[derive(Debug, thiserror::Error)]
pub enum Error<B> {
    /// The underlying byte reader returned an error.
    ///
    /// The original backend error is preserved as the wrapped value.
    #[error("Backend error: {0}")]
    Backend(#[from] B),
    /// The byte stream contained an invalid UTF-8 sequence.
    ///
    /// `offset` is the absolute byte offset (from the start of the stream)
    /// of the first byte of the invalid sequence.
    #[error("invalid UTF-8 at byte offset {offset}")]
    InvalidUtf8 {
        /// The absolute byte offset (from the start of the stream)
        /// of the first byte of the invalid sequence.
        offset: u64,
    },
    /// The internal buffer was too small to hold the required lookahead.
    ///
    /// This is a defensive check: with a buffer capacity of at least 8 bytes
    /// (enforced by [`CharIn::with_capacity`]) it should not occur.
    #[error("Buffer too small (must be >= 8 bytes)")]
    BufferTooSmall,
}
