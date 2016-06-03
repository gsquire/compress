/// Compression modifier for Iron. This simple crate lets you pass bytes to a response such
/// that they can be encoded by either GZIP or deflate algorithms. The modifiers also set the
/// proper content encoding header.

extern crate flate2;
extern crate iron;
extern crate modifier;

use std::io::Write;

use flate2::Compression;
use flate2::write::{DeflateEncoder, GzEncoder};

use iron::Response;
use iron::headers::{ContentEncoding, Encoding};

use modifier::Modifier;

/// A type that knows how to encode a stream of bytes with the deflate algorithm.
pub struct DeflateWriter<'a>(pub &'a [u8]);

/// A type that knows how to encode a stream of bytes with the GZIP algorithm.
pub struct GzipWriter<'a>(pub &'a [u8]);

impl<'a> Modifier<Response> for DeflateWriter<'a> {
    /// Encode the bytes with the deflate algorithm.
    ///
    /// # Panics
    ///
    /// This will panic if the encoder cannot finish writing the stream.
    fn modify(self, res: &mut Response) {
        res.headers.set(ContentEncoding(vec![Encoding::Deflate]));
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::Best);
        let _ = encoder.write_all(self.0);
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes.modify(res);
    }
}

impl<'a> Modifier<Response> for GzipWriter<'a> {
    /// Encode the bytes with the GZIP algorithm.
    ///
    /// # Panics
    ///
    /// This will panic if the encoder cannot finish writing the stream.
    fn modify(self, res: &mut Response) {
        res.headers.set(ContentEncoding(vec![Encoding::Gzip]));
        let mut encoder = GzEncoder::new(Vec::new(), Compression::Best);
        let _ = encoder.write_all(self.0);
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes.modify(res);
    }
}
