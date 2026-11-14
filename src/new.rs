impl<R: crate::io::Read + Unpin> crate::CharIn<R> {
    /// Creates a new `CharIn` with the default buffer capacity (8 KiB).
    ///
    /// # Example
    ///
    /// ```rust
    /// use charry::CharIn;
    /// let mut r = CharIn::new(&b"hi"[..]);
    /// assert_eq!(r.next().unwrap(), Some('h'));
    /// ```
    pub fn new(inner: R) -> Self {
        Self::with_capacity(inner, Self::DEFAULT_CAPACITY)
    }

    /// Creates a new `CharIn` with the given buffer capacity in bytes.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is less than 8. A capacity of 8 is sufficient
    /// for [`CharIn::peek::<2>`](CharIn::peek) to decode any two adjacent
    /// UTF-8 characters.
    pub fn with_capacity(inner: R, capacity: usize) -> Self {
        assert!(capacity >= 8, "capacity must be >= 8");
        Self {
            inner,
            buf: vec![0u8; capacity].into_boxed_slice(),
            pos: 0,
            end: 0,
            eof: false,
            base_offset: 0,
        }
    }
}
