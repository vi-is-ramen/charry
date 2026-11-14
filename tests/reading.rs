use charry::CharIn;

#[test]
fn empty_stream_yields_none() {
    let mut r = CharIn::new(&[][..]);
    assert_eq!(r.next().unwrap(), None);
    assert_eq!(r.peek().unwrap(), [None]);
    assert_eq!(r.peek::<2>().unwrap(), [None, None]);
}

#[test]
fn ascii_roundtrip() {
    let mut r = CharIn::new(&b"hello"[..]);
    let mut out = String::new();
    while let Some(c) = r.next().unwrap() {
        out.push(c);
    }
    assert_eq!(out, "hello");
}

#[test]
fn decodes_multibyte_chars() {
    let mut r = CharIn::new("aé€😀".as_bytes());
    assert_eq!(r.next().unwrap(), Some('a'));
    assert_eq!(r.next().unwrap(), Some('é'));
    assert_eq!(r.next().unwrap(), Some('€'));
    assert_eq!(r.next().unwrap(), Some('😀'));
    assert_eq!(r.next().unwrap(), None);
}
