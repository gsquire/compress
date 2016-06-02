//! Compression modifier for Iron.

extern crate flate2;
extern crate iron;
extern crate modifier;

use std::io::Write;

use flate2::Compression;
use flate2::write::{DeflateEncoder, GzEncoder};

use iron::Response;
use iron::headers::{ContentEncoding, Encoding};

use modifier::Modifier;

pub struct DeflateWriter<'a>(pub &'a [u8]);
pub struct GzipWriter<'a>(pub &'a [u8]);

impl<'a> Modifier<Response> for DeflateWriter<'a> {
    fn modify(self, res: &mut Response) {
        res.headers.set(ContentEncoding(vec![Encoding::Deflate]));
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::Best);
        let _ = encoder.write_all(self.0);
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes.modify(res);
    }
}

impl<'a> Modifier<Response> for GzipWriter<'a> {
    fn modify(self, res: &mut Response) {
        res.headers.set(ContentEncoding(vec![Encoding::Gzip]));
        let mut encoder = GzEncoder::new(Vec::new(), Compression::Best);
        let _ = encoder.write_all(self.0);
        let compressed_bytes = encoder.finish().unwrap();
        compressed_bytes.modify(res);
    }
}
