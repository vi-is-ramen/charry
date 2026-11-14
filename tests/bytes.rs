use charry::CharIn;

#[test]
fn bnext_reads_bytes() {
    let mut r = CharIn::new(&b"hello"[..]);
    assert_eq!(r.bnext().unwrap(), Some(b'h'));
    assert_eq!(r.bnext().unwrap(), Some(b'e'));
    assert_eq!(r.bnext().unwrap(), Some(b'l'));
    assert_eq!(r.bnext().unwrap(), Some(b'l'));
    assert_eq!(r.bnext().unwrap(), Some(b'o'));
    assert_eq!(r.bnext().unwrap(), None);
}

#[test]
fn bpeek1_does_not_advance() {
    let mut r = CharIn::new(&b"ab"[..]);
    assert_eq!(r.bpeek().unwrap(), [Some(b'a')]);
    assert_eq!(r.offset(), 0);
    assert_eq!(r.bnext().unwrap(), Some(b'a'));
    assert_eq!(r.offset(), 1);
}

#[test]
fn bpeek_returns_array() {
    let mut r = CharIn::new(&b"abc"[..]);
    assert_eq!(
        r.bpeek::<3>().unwrap(),
        [Some(b'a'), Some(b'b'), Some(b'c')]
    );
    assert_eq!(r.offset(), 0);
}

#[test]
fn byte_methods_work_with_multibyte() {
    let mut r = CharIn::new("é".as_bytes()); // 0xC3 0xA9
    assert_eq!(r.bnext().unwrap(), Some(0xC3));
    assert_eq!(r.bnext().unwrap(), Some(0xA9));
    assert_eq!(r.bnext().unwrap(), None);
}

#[test]
fn bpeek_partial_eof() {
    let mut r = CharIn::with_capacity(&b"hello"[..], 8);
    assert_eq!(
        r.bpeek::<10>().unwrap(),
        [
            Some(b'h'),
            Some(b'e'),
            Some(b'l'),
            Some(b'l'),
            Some(b'o'),
            None,
            None,
            None,
            None,
            None
        ]
    );
}
