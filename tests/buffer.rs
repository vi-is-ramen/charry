mod common;
use charry::CharIn;

#[test]
fn fragmented_reads_decode_correctly() {
    let src = common::Chunked {
        data: "aé€😀".as_bytes(),
        chunk: 1,
    };
    let mut r = CharIn::new(src);
    let mut out = String::new();
    while let Some(c) = r.next().unwrap() {
        out.push(c);
    }
    assert_eq!(out, "aé€😀");
}

#[test]
fn long_stream_exercises_compaction() {
    let data = vec![b'x'; 1000];
    let mut r = CharIn::with_capacity(&data[..], 8);
    let mut count = 0u64;
    while let Some(c) = r.next().unwrap() {
        assert_eq!(c, 'x');
        count += 1;
    }
    assert_eq!(count, 1000);
    assert_eq!(r.offset(), 1000);
}

#[test]
fn small_capacity_still_handles_peek2() {
    let mut r = CharIn::with_capacity("😀😀".as_bytes(), 8);
    assert_eq!(r.peek::<2>().unwrap(), [Some('😀'), Some('😀')]);
}

#[test]
#[should_panic(expected = "capacity must be >= 8")]
fn with_capacity_below_8_panics() {
    let _ = CharIn::with_capacity(&[][..], 4);
}
