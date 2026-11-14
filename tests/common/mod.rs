#![allow(unused)]
// ^--- False-positive from rustc & clippy

use std::io;

pub struct Chunked<'a> {
    pub data: &'a [u8],
    pub chunk: usize,
}

impl io::Read for Chunked<'_> {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        let n = out.len().min(self.chunk).min(self.data.len());
        out[..n].copy_from_slice(&self.data[..n]);
        self.data = &self.data[n..];
        Ok(n)
    }
}

pub struct FailAfterOnce {
    pub data: Vec<u8>,
    pub used: bool,
}

impl io::Read for FailAfterOnce {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        if self.used {
            return Err(io::Error::other("backend failure"));
        }
        self.used = true;
        let n = out.len().min(self.data.len());
        out[..n].copy_from_slice(&self.data[..n]);
        Ok(n)
    }
}
