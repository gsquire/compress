//! Compression middleware for Iron.

extern crate flate2;
extern crate iron;

use std::io::Write;

use flate2::Compression;
use flate2::write::{DeflateEncoder, GzEncoder};

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
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        let encodings = req.headers.get::<AcceptEncoding>();
        let mut can_encode: bool = false;
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

                let mut def_enc = DeflateEncoder::new(vec![], Compression::Best);
                let _ = def_enc.write(b"how do i read the body?");
                let bytes = def_enc.finish().unwrap();
                res.body = Some(Box::new(bytes));
            },
            Type::Gzip => {
                ce_opts.push(Encoding::Gzip);
                res.headers.set(ContentEncoding(ce_opts));

                let mut gz_enc = GzEncoder::new(vec![], Compression::Best);
                let _ = gz_enc.write(b"how do i read the body?");
                let bytes = gz_enc.finish().unwrap();
                res.body = Some(Box::new(bytes));
            }
        }

        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        Err(err)
    }
}
