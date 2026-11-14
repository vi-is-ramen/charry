mod common;
use charry::{CharIn, Error};

/// End-to-end scenario covering all public APIs, error conditions,
/// buffer compaction, and EOF behavior in a single logical flow.
#[test]
fn comprehensive_e2e_api_usage() {
    // Stream: "aé😀" + [0xFF] + "b"
    // Bytes: 'a'(1) | 'é'(2: C3 A9) | '😀'(4: F0 9F 98 80) | 0xFF(1) | 'b'(1)
    let stream: &[u8] = &[b'a', 0xC3, 0xA9, 0xF0, 0x9F, 0x98, 0x80, 0xFF, b'b'];

    // Force maximum fragmentation (chunk=1) with a small buffer (cap=8)
    // to ensure `ensure_bytes` and `compact` are thoroughly exercised.
    let src = common::Chunked {
        data: stream,
        chunk: 1,
    };
    let mut r = CharIn::with_capacity(src, 8);

    // 1. Initial state
    assert_eq!(r.offset(), 0);
    assert_eq!(format!("{:?}", r), "CharIn { .. }");

    // 2. Character-level lookahead (peek)
    assert_eq!(r.peek::<3>().unwrap(), [Some('a'), Some('é'), Some('😀')]);
    assert_eq!(r.offset(), 0); // Peek must not consume

    // 3. Byte-level lookahead (bpeek)
    assert_eq!(r.bpeek::<2>().unwrap(), [Some(b'a'), Some(0xC3)]);
    assert_eq!(r.offset(), 0);

    // 4. Byte-level reading (bnext)
    assert_eq!(r.bpeek().unwrap(), [Some(b'a')]);
    assert_eq!(r.bnext().unwrap(), Some(b'a'));
    assert_eq!(r.offset(), 1);

    // 5. Character reading (next)
    assert_eq!(r.peek().unwrap(), [Some('é')]);
    assert_eq!(r.next().unwrap(), Some('é'));
    assert_eq!(r.offset(), 3);

    // 6. Skip over a multi-byte character
    assert_eq!(r.peek().unwrap(), [Some('😀')]);
    r.skip(4).unwrap();
    assert_eq!(r.offset(), 7);

    // 7. Error handling: Invalid UTF-8
    match r.next() {
        Err(Error::InvalidUtf8 { offset }) => assert_eq!(offset, 7),
        other => panic!("expected InvalidUtf8 at offset 7, got {other:?}"),
    }
    assert_eq!(r.offset(), 8); // Reader advances past the bad byte

    // 8. Reading after error recovery
    assert_eq!(r.next().unwrap(), Some('b'));
    assert_eq!(r.offset(), 9);

    // 9. EOF behavior across all methods
    assert_eq!(r.next().unwrap(), None);
    assert_eq!(r.peek().unwrap(), [None]);
    assert_eq!(r.bnext().unwrap(), None);
    assert_eq!(r.bpeek().unwrap(), [None]);
    assert_eq!(r.peek::<3>().unwrap(), [None, None, None]);
    assert_eq!(r.bpeek::<2>().unwrap(), [None, None]);

    // 10. Skipping past EOF is safe and stops at the end
    r.skip(100).unwrap();
    assert_eq!(r.offset(), 9);
    assert_eq!(r.next().unwrap(), None);
}

/// Secondary e2e scenario targeting the remaining error variants:
/// `Error::Backend` and `Error::BufferTooSmall`.
#[test]
fn e2e_error_variants() {
    // 1. Backend error propagation
    let fail_src = common::FailAfterOnce {
        data: vec![b'x'; 8],
        used: false,
    };
    let mut r1 = CharIn::with_capacity(fail_src, 8);

    for _ in 0..8 {
        assert_eq!(r1.next().unwrap(), Some('x'));
    }

    match r1.next() {
        Err(Error::Backend(e)) => assert!(e.to_string().contains("backend failure")),
        other => panic!("expected Backend error, got {other:?}"),
    }

    // 2. Buffer capacity limit
    // Three 4-byte emojis require 12 bytes, but capacity is 8.
    let mut r2 = CharIn::with_capacity("😀😀😀".as_bytes(), 8);
    match r2.peek::<3>() {
        Err(Error::BufferTooSmall) => {}
        other => panic!("expected BufferTooSmall, got {other:?}"),
    }

    // Reader remains usable after BufferTooSmall
    assert_eq!(r2.next().unwrap(), Some('😀'));
}
