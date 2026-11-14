impl<R: crate::io::Read + Unpin> core::fmt::Debug for crate::CharIn<R> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "CharIn {{ .. }}")
    }
}
