# charry

[![Crates.io](https://img.shields.io/crates/v/charry.svg)](https://crates.io/crates/charry)
[![Docs.rs](https://docs.rs/charry/badge.svg)](https://docs.rs/charry)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![CI](https://github.com/vi-is-ramen/charry/actions/workflows/ci.yml/badge.svg)](https://github.com/vi-is-ramen/charry/actions/workflows/ci.yml)
[![Coverage](https://vi-is-ramen.github.io/charry/cov/latest/badge.svg)](https://vi-is-ramen.github.io/charry/cov/latest/)

`charry` provides [`CharIn`], a buffered UTF-8 decoder that wraps any byte-oriented
reader and yields [`char`]s. It works in both `std` and `no_std` (with `alloc`)
environments, and reports invalid UTF-8 sequences precisely — down to the byte
offset where the bad sequence begins.

## Features

- **UTF-8 aware**: decodes multi-byte UTF-8 sequences, even when the backend
  returns bytes in fragments.
- **Lookahead**: `peek`/`peek::<N>` let you inspect upcoming characters without
  consuming them.
- **Precise errors**: invalid UTF-8 is reported with its absolute byte offset,
  and the reader advances past the offending byte so you can recover and keep
  reading.
- **`no_std` friendly**: only requires `alloc`.
- **Bring your own reader**: anything implementing `std::io::Read` (files,
  sockets, `&[u8]`, etc.) works out of the box.

## Installation

Add `charry` to your `Cargo.toml`:

```toml
[dependencies]
charry = ".."
```

To use `charry` in a `no_std` environment, disable the default `std` feature:

```toml
[dependencies]
charry = { version = "..", default-features = false }
```

## Usage

```rust
use charry::CharIn;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = CharIn::new(&b"hello, world"[..]);

    while let Some(c) = reader.next()? {
        print!("{c}");
    }
    println!();

    Ok(())
}
```

### Lookahead

```rust
use charry::CharIn;

let mut reader = CharIn::new(&b"abc"[..]);

// Peek without consuming.
assert_eq!(reader.peek()?, [Some('a')]);
assert_eq!(reader.peek()?, [Some('a'), Some('b')]);

// The characters are still available.
assert_eq!(reader.next()?, Some('a'));
assert_eq!(reader.next()?, Some('b'));
assert_eq!(reader.next()?, Some('c'));
assert_eq!(reader.next()?, None);
# Ok::<(), charry::Error<std::io::Error>>(())
```

### Peeking N characters

`peek::<N>` returns the next `N` characters at once. Entries past the end of
the stream are `None`.

```rust
use charry::CharIn;

let mut reader = CharIn::new("aé😀z".as_bytes());

assert_eq!(
    reader.peek::<4>()?,
    [Some('a'), Some('é'), Some('😀'), Some('z')],
);

// Near the end of the stream, missing characters are `None`.
let mut short = CharIn::new("hi".as_bytes());
assert_eq!(
    short.peek()?,
    [Some('h'), Some('i'), None, None],
);
# Ok::<(), charry::Error<std::io::Error>>(())
```

`peek` may need to compact or refill the internal buffer. If the requested
window cannot fit in the buffer, it returns [`Error::BufferTooSmall`]. The
default 8 KiB buffer is more than enough for any practical `N`.

### Handling invalid UTF-8

```rust
use charry::CharIn;

// 0xFF is not a valid UTF-8 lead byte.
let mut reader = CharIn::new(&[b'a', 0xFF, b'b'][..]);

assert_eq!(reader.next()?, Some('a'));

// The bad byte is reported with its offset...
match reader.next() {
    Err(charry::Error::InvalidUtf8 { offset }) => assert_eq!(offset, 1),
    other => panic!("expected InvalidUtf8, got {other:?}"),
}

// ...and the reader resumes after it.
assert_eq!(reader.next()?, Some('b'));
# Ok::<(), charry::Error<std::io::Error>>(())
```

### Custom buffer capacity

```rust
use charry::CharIn;

// Default capacity is 8 KiB; you can pick your own (must be >= 8).
let mut reader = CharIn::with_capacity(&b"hi"[..], 64);
assert_eq!(reader.next()?, Some('h'));
# Ok::<(), charry::Error<std::io::Error>>(())
```

## API overview

| Item | Description |
| --- | --- |
| `CharIn<R>` | Buffered UTF-8 character reader over a byte source. |
| `CharIn::new(inner)` | Create with the default 8 KiB buffer. |
| `CharIn::with_capacity(inner, cap)` | Create with a custom buffer (&ge; 8 bytes). |
| `CharIn::next()` | Read the next character. |
| `CharIn::bnext()` | Read the next byte (bit octet). |
| `CharIn::peek::<N>()` | Look at the next `N` characters without consuming them. |
| `CharIn::bpeek::<N>()` | Look at the next `N` bytes (bit octets) without consuming them. |
| `CharIn::offset()` | Byte offset of the next character to be read. |
| `CharIn::compact()` | Compact inner buffer. Useful before heavy peek operations. |
| `CharIn::skip(amount)` | Skip `amount` bytes (bit octets) from the input. |
| `Error<B>` | Backend error, invalid UTF-8 (with offset), or buffer-too-small. |
| `io::Read` | Byte-source trait; blanket-impl'd for `std::io::Read` on `std`. |

## `no_std` support

`charry` is `#![no_std]` when the `std` feature is disabled, and only depends
on `alloc`. In this mode the blanket `io::Read` impl for `std::io::Read` is
omitted; implement `charry::io::Read` for your own byte source instead:

```rust,ignore
use charry::io;

struct MySource { /* ... */ }

impl io::Read for MySource {
    type Error = core::convert::Infallible;

    fn read(&mut self, out: &mut [u8]) -> Result<usize, Self::Error> {
        // fill `out`, return bytes written
        Ok(0)
    }
}
```

When `core::io::Read` stabilizes, the `std` gate will be removed and the
trait will alias the core type directly.

## MSRV

The minimum supported Rust version is **1.85** (edition 2024).

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
