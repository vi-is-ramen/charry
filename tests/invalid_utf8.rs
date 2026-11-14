use charry::{CharIn, Error};

#[test]
fn invalid_utf8_reports_offset_and_recovers() {
    let mut r = CharIn::new(&[b'a', 0xFF, b'b'][..]);
    assert_eq!(r.next().unwrap(), Some('a'));
    match r.next() {
        Err(Error::InvalidUtf8 { offset }) => assert_eq!(offset, 1),
        other => panic!("expected InvalidUtf8, got {other:?}"),
    }
    assert_eq!(r.next().unwrap(), Some('b'));
}

#[test]
fn truncated_multibyte_at_eof_is_invalid() {
    let mut r = CharIn::new(&[b'a', 0xE2, 0x82][..]);
    assert_eq!(r.next().unwrap(), Some('a'));
    match r.next() {
        Err(Error::InvalidUtf8 { offset }) => assert_eq!(offset, 1),
        other => panic!("expected InvalidUtf8, got {other:?}"),
    }
}

#[test]
fn overlong_encoding_is_invalid() {
    let mut r = CharIn::new(&[0xC0, 0x80][..]);
    match r.next() {
        Err(Error::InvalidUtf8 { offset }) => assert_eq!(offset, 0),
        other => panic!("expected InvalidUtf8, got {other:?}"),
    }
}
