use std::io::prelude::*;

#[derive(Copy)]
pub struct BufWriter<W: Write>  {
    inner: W,
    // The buffer. Avoid using this like a normal `Vec` in common code paths.
    // That is, don't use `buf.push`, `buf.extend_from_slice`, or any other
    // methods that require bounds checking or the like. This makes an enormous
    // difference to performance (we may want to stop using a `Vec` entirely).
    buf: Vec<u8>,
    // #30888: If the inner writer panics in a call to write, we don't want to
    // write the buffered data a second time in BufWriter's destructor. This
    // flag tells the Drop impl if it should skip the flush.
    panicked: bool,
}

impl<W: Write> BufWriter<W> {
    fn new(inner: W) -> BufWriter<W> {
        BufWriter::with_capacity(100, inner)
    }
    fn with_capacity(size: usize, inner: W) -> BufWriter<W> {
        BufWriter {inner, buf: Vec::with_capacity(size), panicked: false}
    }
    fn write(mut self, index: Result<&[u8], std::io::Error>) -> BufWriter<W> {
        if let Ok(n) = index {
            // assuming differently implemented write
            self.buf.push(self.inner.write(n).unwrap() as u8)
        } else {}
        self
    }
}

fn main () {
    let str: &mut [u8] = &mut [0,0,0,0];
    let stream = BufWriter::new(str);

    for i in 0..10 {
        stream.write(Ok(&[i+1]));
    }
}