extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate http;

//use actix::Arbiter;
use actix_web::*;
use actix_web::client::*;
use bytes::Bytes;
use futures::Future;
use futures::Stream;
use futures::future::{FutureResult, result};
use futures::stream::once;
//use std::io::Write;
use std::io::{self, ErrorKind};

mod apperror;

use apperror::*;


fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn async_forward(_req: HttpRequest) -> Box<Future<Item=HttpResponse, Error=AppError>> {
    client::ClientRequest::get("http://www.liveviewtech.com/")
        .finish().unwrap()
        .send()                         // <- connect to host and send request
        .map_err(apperror::AppError::from)    // <- convert SendRequestError to an Error
        .and_then(|resp| {              // <- we received client response
            httpcodes::HttpOk.build()
            // read one chunk from client response and send this chunk to a server response
            // .from_err() converts PayloadError to a Error
                .body(Body::Streaming(Box::new(resp.from_err())))
                .map_err(|e| e.into()) // HttpOk::build() mayb return HttpError, we need to convert it to a Error
        })
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();
    let sys = actix::System::new("ws-example");

    let _addr = HttpServer::new(
        || Application::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/async", |r| r.route().f(async_forward))
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index)))
        .threads(4)
        .bind("127.0.0.1:8080").unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
