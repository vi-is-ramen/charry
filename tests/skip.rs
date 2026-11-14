use charry::CharIn;

#[test]
fn skip_advances_by_bytes() {
    let mut r = CharIn::new(&b"hello"[..]);
    r.skip(3).unwrap();
    assert_eq!(r.offset(), 3);
    assert_eq!(r.bnext().unwrap(), Some(b'l'));
}

#[test]
fn skip_does_not_decode() {
    let mut r = CharIn::new(&[0xC3, 0xA9, b'a'][..]); // "éa"
    r.skip(1).unwrap(); // Skip half of 'é'
    assert_eq!(r.offset(), 1);
    assert_eq!(r.bnext().unwrap(), Some(0xA9));
    assert_eq!(r.bnext().unwrap(), Some(b'a'));
}

#[test]
fn skip_over_invalid_utf8() {
    let mut r = CharIn::new(&[0xFF, b'a'][..]);
    r.skip(1).unwrap();
    assert_eq!(r.offset(), 1);
    assert_eq!(r.next().unwrap(), Some('a')); // Reader recovers gracefully
}

#[test]
fn skip_past_eof() {
    let mut r = CharIn::new(&b"hi"[..]);
    r.skip(10).unwrap();
    assert_eq!(r.offset(), 2); // Stops at EOF, doesn't panic
    assert_eq!(r.next().unwrap(), None);
}
