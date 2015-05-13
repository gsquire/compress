//! Compression middleware for Iron.

extern crate flate2;
extern crate iron;

use std::io::prelude::*;

use flate2::Compression;
use flate2::write::GzEncoder;

use iron::{AfterMiddleware, IronResult, IronError, Request, Response};

/// The EncoderWriter is used from the flate2 crate.
pub struct Compressor {
    engine: GzEncoder<Vec<u8>>
}

impl Compressor {
    pub fn new() -> Compressor {
        Compressor { engine: GzEncoder::new(Vec::new(), Compression::Best) }
    }
}
