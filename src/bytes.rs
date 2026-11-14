impl<R: crate::io::Read + Unpin> crate::CharIn<R> {
    /// Returns the next byte without consuming it.
    ///
    /// Returns `Ok(None)` at end of stream.
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    pub(crate) fn bpeek1(&mut self) -> crate::Result<Option<u8>, R::Error> {
        let mut cursor = self.pos;
        self.ensure_bytes(&mut cursor, 1)?;
        Ok(self.buf[..self.end].get(self.pos).copied())
    }

    /// Returns up to `N` bytes without consuming them.
    ///
    /// The returned array contains the next `N` bytes in order. If the
    /// stream ends before `N` bytes can be read, the trailing entries
    /// are [`None`].
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    /// - [`Error::BufferTooSmall`] if the bytes required to peek `N`
    ///   bytes cannot fit in the internal buffer, even after compaction.
    pub fn bpeek<const N: usize>(&mut self) -> crate::Result<[Option<u8>; N], R::Error> {
        let mut result = [None; N];
        if N == 0 {
            return Ok(result);
        }

        result[0] = self.bpeek1()?;
        if result[0].is_none() {
            return Ok(result);
        }

        let mut cursor = self.pos + 1;
        for slot in &mut result[1..] {
            match self.ensure_bytes(&mut cursor, 1) {
                Ok(..) => {
                    *slot = self.buf[..self.end].get(cursor).copied();
                    if slot.is_none() {
                        break; // EOF
                    }
                    cursor += 1;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(result)
    }

    /// Reads the next byte, advancing the reader past it.
    ///
    /// Returns `Ok(None)` at end of stream.
    ///
    /// # Errors
    ///
    /// - [`Error::Backend`] if the underlying reader fails.
    #[allow(clippy::should_implement_trait)]
    pub fn bnext(&mut self) -> crate::Result<Option<u8>, R::Error> {
        match self.bpeek1() {
            Ok(Some(b)) => {
                self.pos += 1;
                Ok(Some(b))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
