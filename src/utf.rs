/// Result of the character decoding attempt.
pub enum DecodeStep {
    /// Success.
    Valid(char),
    /// Need more bytes to decode.
    NeedMore(usize),
    /// EOF truncated the character.
    TruncatedEof,
    /// Invalid UTF-8 byte sequence.
    Invalid(u64),
}

/// Returns the length in bytes of a UTF-8 sequence given its first byte.
///
/// Returns `1` for any byte that does not begin a multi-byte sequence
/// (including ASCII and invalid leading bytes), so the caller can handle
/// them uniformly.
#[inline]
fn ulen(b: u8) -> usize {
    match b {
        0xC0..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF7 => 4,
        _ => 1,
    }
}

/// Decodes one character from the byte slice at the given cursor position.
///
/// This is a pure function with no side effects — it does not modify the reader state.
///
/// # Arguments
///
/// * `buf` - The byte buffer to decode from.
/// * `cursor` - The starting position in the buffer.
/// * `eof` - Whether end-of-file has been reached.
/// * `base_offset` - The absolute byte offset of the buffer start.
/// * `allow_trunc` - If true, truncated sequences at EOF are treated as `None`
///   instead of errors (used for lookahead operations).
pub fn decode_char(
    buf: &[u8],
    cursor: usize,
    eof: bool,
    base_offset: u64,
    allow_trunc: bool,
) -> DecodeStep {
    let Some(&b) = buf.get(cursor) else {
        return if eof {
            DecodeStep::TruncatedEof
        } else {
            DecodeStep::NeedMore(1)
        };
    };

    // ASCII range
    if b < 0x80 {
        return DecodeStep::Valid(b as char);
    }

    let len = ulen(b);
    if cursor + len > buf.len() {
        return if eof {
            if allow_trunc {
                DecodeStep::TruncatedEof
            } else {
                DecodeStep::Invalid(base_offset + cursor as u64)
            }
        } else {
            DecodeStep::NeedMore(len)
        };
    }

    match core::str::from_utf8(&buf[cursor..cursor + len]) {
        Ok(s) => DecodeStep::Valid(s.chars().next().unwrap()),
        Err(_) => DecodeStep::Invalid(base_offset + cursor as u64),
    }
}
