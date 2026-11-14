impl<R: crate::io::Read + Unpin> crate::CharIn<R> {
    /// Returns the next character without consuming it.
    ///
    /// Returns `Ok(None)` at end of stream.
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    /// - [`Error::InvalidUtf8`] if the upcoming bytes are not valid UTF-8.
    pub(crate) fn peek1(&mut self) -> crate::Result<Option<char>, R::Error> {
        let mut cursor = self.pos;
        self.decode_at(&mut cursor, false)
    }

    /// Returns up to `N` characters without consuming them.
    ///
    /// The returned array contains the next `N` characters in order. If the
    /// stream ends before `N` characters can be read, the trailing entries
    /// are [`None`].
    ///
    /// A truncated multi-byte sequence at end of stream is treated as
    /// "no further complete character available" — the affected slot and all
    /// subsequent slots are [`None`]. Invalid UTF-8 that is *not* due to
    /// truncation at EOF still yields [`Error::InvalidUtf8`].
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    /// - [`Error::InvalidUtf8`] if any of the peeked characters is not valid
    ///   UTF-8. The reported offset is the start of the invalid sequence.
    /// - [`Error::BufferTooSmall`] if the bytes required to peek `N`
    ///   characters cannot fit in the internal buffer, even after compaction.
    pub fn peek<const N: usize>(&mut self) -> crate::Result<[Option<char>; N], R::Error> {
        let mut result = [None; N];
        if N == 0 {
            return Ok(result);
        }

        result[0] = self.peek1()?;
        if result[0].is_none() {
            return Ok(result);
        }

        /*
         * Using unwrap_or_default here keeps the function panic-free even if an
         * invariant is later broken; the code path is unreachable in practice.
         */
        let mut cursor = self.pos + result[0].unwrap_or_default().len_utf8();
        for slot in &mut result[1..] {
            match self.decode_at(&mut cursor, true) {
                Ok(Some(c)) => {
                    *slot = Some(c);
                    cursor += c.len_utf8();
                }
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }

    /// Reads the next character, advancing the reader past it.
    ///
    /// Returns `Ok(None)` at end of stream.
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    /// - [`Error::InvalidUtf8`] if the upcoming bytes are not valid UTF-8.
    ///   In this case the reader advances past the first invalid byte, so
    ///   subsequent calls resume after it.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> crate::Result<Option<char>, R::Error> {
        match self.peek1() {
            Ok(Some(c)) => {
                self.pos += c.len_utf8();
                Ok(Some(c))
            }
            Ok(None) => Ok(None),
            Err(e @ crate::Error::InvalidUtf8 { .. }) => {
                self.pos += 1;
                Err(e)
            }
            Err(e) => Err(e),
        }
    }

    /// State machine that orchestrates character decoding.
    ///
    /// Cycles through `decode_char` calls, requesting more bytes via `ensure_bytes`
    /// when necessary. Does not modify `self.pos` directly.
    pub(crate) fn decode_at(
        &mut self,
        cursor: &mut usize,
        allow_trunc: bool,
    ) -> crate::Result<Option<char>, R::Error> {
        loop {
            // Pass only the filled portion of the buffer (`..self.end`), not
            // the entire allocated capacity, so `buf.get()` correctly returns
            // `None` at the boundary of valid data.
            match crate::decode_char(
                &self.buf[..self.end],
                *cursor,
                self.eof,
                self.base_offset,
                allow_trunc,
            ) {
                crate::DecodeStep::Valid(c) => return Ok(Some(c)),
                crate::DecodeStep::TruncatedEof => return Ok(None),
                crate::DecodeStep::Invalid(off) => {
                    return Err(crate::Error::InvalidUtf8 { offset: off });
                }
                crate::DecodeStep::NeedMore(len) => {
                    self.ensure_bytes(cursor, len)?;
                    if self.eof && *cursor >= self.end {
                        return Ok(None);
                    }
                }
            }
        }
    }
}
