#![feature(async_await, await_macro, futures_api)]

extern crate failure;
extern crate futures;
extern crate hyper;
extern crate tokio;

use failure::Error;
use futures::{compat::*, prelude::*};


fn main() {
    let fut = execute().then(|r| {
        if let Err(e) = r {
            eprintln!("{}", e);
        }
        future::ready(())
    });
    // convert Future03 to Future01
    let fut = fut.unit_error().boxed().compat(TokioDefaultSpawn);
    tokio::run(fut);
}

async fn execute() -> Result<(), Error> {
    let client = hyper::Client::new();

    // convert Future01 to Future03
    let fut = client.get("http://m11c.blog".parse()?).compat();
    let response = await!(fut)?;

    // convert Stream01 to Stream03
    let body = response.into_body().compat();
    let content: Vec<_> = await!(body.try_collect())?;

    for chunk in content {
        print!("{}", std::str::from_utf8(&chunk)?);
    }

    Ok(())
}
