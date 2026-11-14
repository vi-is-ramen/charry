mod common;
use charry::{CharIn, Error};

#[test]
fn peek_does_not_advance() {
    let mut r = CharIn::new("é".as_bytes());
    assert_eq!(r.peek().unwrap(), [Some('é')]);
    assert_eq!(r.peek().unwrap(), [Some('é')]);
    assert_eq!(r.offset(), 0);
    assert_eq!(r.next().unwrap(), Some('é'));
    assert_eq!(r.offset(), 2);
}

#[test]
fn peek2_returns_pair_then_consumes() {
    let mut r = CharIn::new("ab".as_bytes());
    assert_eq!(r.peek::<2>().unwrap(), [Some('a'), Some('b')]);
    assert_eq!(r.next().unwrap(), Some('a'));
    assert_eq!(r.next().unwrap(), Some('b'));
    assert_eq!(r.next().unwrap(), None);
}

#[test]
fn peek2_with_single_char_left_returns_some_none() {
    let mut r = CharIn::new("a".as_bytes());
    assert_eq!(r.peek::<2>().unwrap(), [Some('a'), None]);
    assert_eq!(r.next().unwrap(), Some('a'));
}

#[test]
fn offset_tracks_bytes() {
    let mut r = CharIn::new("aé😀b".as_bytes());
    assert_eq!(r.offset(), 0);
    assert_eq!(r.next().unwrap(), Some('a'));
    assert_eq!(r.offset(), 1);
    assert_eq!(r.next().unwrap(), Some('é'));
    assert_eq!(r.offset(), 3);
    assert_eq!(r.next().unwrap(), Some('😀'));
    assert_eq!(r.offset(), 7);
    assert_eq!(r.next().unwrap(), Some('b'));
    assert_eq!(r.offset(), 8);
}

#[test]
fn peek_n_zero() {
    let mut r = CharIn::new("abc".as_bytes());
    let empty: [Option<char>; 0] = r.peek::<0>().unwrap();
    assert_eq!(empty, []);
    assert_eq!(r.offset(), 0);
}

#[test]
fn peek_n_basic() {
    let mut r = CharIn::new("abcde".as_bytes());
    assert_eq!(r.peek::<3>().unwrap(), [Some('a'), Some('b'), Some('c')]);
    assert_eq!(r.offset(), 0);
    assert_eq!(r.next().unwrap(), Some('a'));
    assert_eq!(r.offset(), 1);
}

#[test]
fn peek_n_partial_eof() {
    let mut r = CharIn::new("ab".as_bytes());
    assert_eq!(
        r.peek::<5>().unwrap(),
        [Some('a'), Some('b'), None, None, None]
    );
    assert_eq!(r.next().unwrap(), Some('a'));
}

#[test]
fn peek_n_multibyte() {
    let mut r = CharIn::new("aé😀".as_bytes());
    assert_eq!(r.peek::<3>().unwrap(), [Some('a'), Some('é'), Some('😀')]);
}

#[test]
fn peek_n_invalid_utf8() {
    let mut r = CharIn::new(&[b'a', 0xFF, b'b'][..]);
    match r.peek::<2>() {
        Err(Error::InvalidUtf8 { offset }) => assert_eq!(offset, 1),
        other => panic!("expected InvalidUtf8, got {other:?}"),
    }
    assert_eq!(r.offset(), 0); // Peek must not consume on error
}

#[test]
fn peek_n_buffer_too_small() {
    let mut r = CharIn::with_capacity("😀😀😀".as_bytes(), 8);
    match r.peek::<3>() {
        Err(Error::BufferTooSmall) => {}
        other => panic!("expected BufferTooSmall, got {other:?}"),
    }
}

#[test]
fn peek_n_compacts_when_needed() {
    let mut r = CharIn::with_capacity("😀😀😀".as_bytes(), 8);
    assert_eq!(r.next().unwrap(), Some('😀')); // pos = 4
    assert_eq!(r.peek::<2>().unwrap(), [Some('😀'), Some('😀')]);
}

#[test]
fn peek_n_fragmented() {
    let src = common::Chunked {
        data: "aé😀".as_bytes(),
        chunk: 1,
    };
    let mut r = CharIn::new(src);
    assert_eq!(r.peek::<3>().unwrap(), [Some('a'), Some('é'), Some('😀')]);
}

#[test]
fn peek_n_after_consumption() {
    let mut r = CharIn::new("aé😀b".as_bytes());
    assert_eq!(r.next().unwrap(), Some('a'));
    assert_eq!(r.peek::<3>().unwrap(), [Some('é'), Some('😀'), Some('b')]);
}
