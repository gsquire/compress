//! Compression middleware for Iron.

extern crate flate2;
extern crate iron;

use std::io::Read;

use flate2::Compression;
use flate2::read::{DeflateEncoder, GzEncoder};

use iron::{AfterMiddleware, IronResult, IronError, Request, Response};
use iron::headers::{AcceptEncoding, ContentEncoding, Encoding};

#[derive(Clone, PartialEq, PartialOrd)]
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
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<(Response)> {
        let encodings = req.headers.get::<AcceptEncoding>();
        let can_encode: bool = false;
        match encodings {
            None => { return Ok(res); }
            Some(encodings) => {
                for enc in encodings.iter() {
                    if enc.item == Encoding::Gzip && self.engine == Type::Gzip
                        || enc.item == Encoding::Deflate && self.engine == Type::Deflate {
                            can_encode = true;
                            break;
                    }
                }
                if !can_encode {
                    return Ok(res);
                }
            }
        }

        let mut ce_opts = Vec::new();

        match self.engine {
            Type::Deflate => {
                ce_opts.push(Encoding::Deflate);
                res.headers.set(ContentEncoding(ce_opts));
                res.body = Some(Box::new(DeflateEncoder::new(res.body.unwrap(), Compression::Best)));
            },
            Type::Gzip => {
                ce_opts.push(Encoding::Gzip);
                res.headers.set(ContentEncoding(ce_opts));
                res.body = Some(Box::new(GzEncoder::new(res.body.unwrap(), Compression::Best)));
            }
        }

        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        Err(err)
    }
}
