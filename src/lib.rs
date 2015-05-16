//! Compression middleware for Iron.

extern crate flate2;
extern crate iron;

use std::io::prelude::*;

use flate2::Compression;
use flate2::write::GzEncoder;

use iron::{AfterMiddleware, IronResult, IronError, Request, Response};
use iron::headers::{ContentEncoding, Encoding};

#[derive(Clone)]
pub enum Type {
    Deflate,
    Gzip
}

/// The EncoderWriter is used from the flate2 crate.
pub struct Compressor {
    engine: Type
}

impl Compressor {
    pub fn new(compression: Type) -> Compressor {
        Compressor { engine: compression }
    }
}

impl AfterMiddleware for Compressor {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<(Response)> {
        let mut ce_opts = Vec::new();

        match self.engine {
            Type::Deflate => {
                ce_opts.push(Encoding::Deflate);
                res.headers.set(ContentEncoding(ce_opts));
            },
            Type::Gzip => {
                ce_opts.push(Encoding::Gzip);
                res.headers.set(ContentEncoding(ce_opts));
            }
        }

        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        Err(err)
    }
}
