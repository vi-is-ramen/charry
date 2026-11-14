mod common;
use charry::{CharIn, Error};

#[test]
fn backend_error_is_propagated() {
    let src = common::FailAfterOnce {
        data: vec![b'a'; 8],
        used: false,
    };
    let mut r = CharIn::with_capacity(src, 8);
    for _ in 0..8 {
        assert_eq!(r.next().unwrap(), Some('a'));
    }
    match r.next() {
        Err(Error::Backend(e)) => assert!(e.to_string().contains("backend failure")),
        other => panic!("expected Backend, got {other:?}"),
    }
}
