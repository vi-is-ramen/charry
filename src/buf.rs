impl<R: crate::io::Read + Unpin> crate::CharIn<R> {
    /// Moves unconsumed bytes to the front of the buffer.
    #[inline]
    pub fn compact(&mut self) {
        if self.pos > 0 {
            let remaining = self.end - self.pos;
            self.buf.copy_within(self.pos..self.end, 0);
            self.base_offset += self.pos as u64;
            self.pos = 0;
            self.end = remaining;
        }
    }

    /// Returns the buffered byte window without consuming it.
    ///
    /// When the window is exhausted (`pos == end`) and the stream is not at
    /// EOF, the buffer is compacted and refilled toward capacity, so callers
    /// that scan the slice directly (lexers, parsers) get large windows and
    /// pay the backend cost once per window instead of once per byte.
    ///
    /// Pair with [`CharIn::consume`] to advance past scanned bytes. The
    /// returned slice is invalidated by any subsequent mutating call
    /// (`consume`, `skip`, `next`, `bnext`, `compact`, ...).
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    #[inline]
    pub fn fill_buf(&mut self) -> crate::Result<&[u8], R::Error> {
        if self.pos == self.end && !self.eof {
            self.compact();
            let cap = self.buf.len();
            let mut cursor = 0;
            self.ensure_bytes(&mut cursor, cap)?;
        }
        Ok(&self.buf[self.pos..self.end])
    }

    /// Advances past `n` bytes of the current window.
    ///
    /// `n` must not exceed the length of the slice returned by the last
    /// [`CharIn::fill_buf`] call. Unlike [`CharIn::skip`], this never reads
    /// from the backend: the bytes are already buffered.
    ///
    /// # Panics
    ///
    /// Debug-asserts that `n <= end - pos`.
    #[inline]
    pub fn consume(&mut self, n: usize) {
        debug_assert!(n <= self.end - self.pos, "consume past window");
        self.pos += n;
    }

    /// Ensures at least `needed` bytes are available starting from `cursor`.
    ///
    /// Handles buffer compaction when necessary and reads from the backend
    /// in a loop until enough bytes are available or EOF is reached.
    ///
    /// # Errors
    ///
    /// - [`Error::BufferTooSmall`] if compaction cannot free enough space.
    /// - [`Error::Backend`] if the underlying reader fails.
    pub(crate) fn ensure_bytes(
        &mut self,
        cursor: &mut usize,
        needed: usize,
    ) -> crate::Result<(), R::Error> {
        if *cursor + needed > self.buf.len() {
            if self.pos == 0 {
                return Err(crate::Error::BufferTooSmall);
            }
            let shift = self.pos;
            self.compact();
            *cursor -= shift;
            /* Check again after compaction.
             * If it still doesn't fit, the buffer is genuinely too small for
             * this lookahead.
             */
            if *cursor + needed > self.buf.len() {
                return Err(crate::Error::BufferTooSmall);
            }
        }

        while *cursor + needed > self.end {
            if self.eof {
                break;
            }
            let n = self.inner.read(&mut self.buf[self.end..])?;
            if n == 0 {
                self.eof = true;
                break;
            }
            self.end += n;
        }
        Ok(())
    }

    /// Skips exact number of bytes from the source. Useful to get rid of
    /// duplicated decoding:
    ///
    /// ```rust
    /// # use charry::CharIn;
    /// // BAD
    /// let mut charin = CharIn::new(&b"\xD1\x91llo"[..]);
    ///
    /// let _ = match charin.peek() {
    ///     Ok([Some(c)]) => charin.next(),
    ///     // ^--- BAD: second decoding!
    ///     Err(e) => Err(e),
    ///     _ => Ok(None),
    /// }
    /// # ;
    /// ```
    /// ---
    /// ```rust
    /// # use charry::CharIn;
    /// // GOOD
    /// let mut charin = CharIn::new(&b"\xD1\x91llo"[..]);
    ///
    /// let _ = match charin.peek() {
    ///     Ok([Some(c)]) => charin.skip(c.len_utf8()),
    ///     // ^--- GOOD: skip second decoding!
    ///     Err(e) => Err(e),
    ///     _ => Ok(()),
    /// }
    /// # ;
    /// ```
    ///
    /// # Errors
    ///
    /// - [`Error::BufferTooSmall`] if compaction cannot free enough space.
    /// - [`Error::Backend`] if the underlying reader fails.
    pub fn skip(&mut self, amount: usize) -> crate::Result<(), R::Error> {
        /* explicit return is unnecessary: `while left > 0` already does it. */
        // if amount == 0 {
        //     return Ok(());
        // }
        let mut left = amount;
        while left > 0 {
            if self.pos == self.end {
                if self.eof {
                    break; // Cannot skip past actual EOF
                }
                let mut cursor = self.pos;
                self.ensure_bytes(&mut cursor, 1)?;
                self.pos = cursor;
                continue;
            }
            let available = self.end - self.pos;
            let to_skip = core::cmp::min(available, left);
            self.pos += to_skip;
            left -= to_skip;
        }
        Ok(())
    }
}
