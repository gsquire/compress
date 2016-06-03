# compress
This small library exposes two types that know how to encode bytes based on the GZIP and deflate
algorithms. The types implement the modifier trait so they can be used in an Iron Response.

### Example
```rust
extern crate iron;
extern crate iron_compress;

use iron::prelude::*;
use iron::status;

use iron_compress::GzipWriter;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, GzipWriter(b"Some compressed response"))))
    }

    Iron::new(hello_world).http("localhost:3000").unwrap();
}
```

### License
MIT
