extern crate actix;
extern crate actix_web;
extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate http;

use actix_web::dev::HttpResponseBuilder;
use actix_web::*;
// use bytes::Bytes;
use futures::Future;
// use futures::stream::Stream;
// use futures::IntoFuture;
// use std::path::PathBuf;

mod apperror;

use apperror::*;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn catchall(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = AppError>> {
    let s: String = if let Ok(s) = req.match_info().query("catchall") {
        s
    } else {
        "blah".to_owned() //TODO: return an error instead
    };
    println!("s => {}", s);
    let base_url = "https://cameras.liveviewtech.com/".to_owned();
    let full_url = format!("{}{}", base_url, s);
    let mut client_req;
    {
        client_req = client::ClientRequestBuilder::from(&req);
    }
    client_req.uri(&full_url)
        .streaming(req).unwrap() // TODO: actually handle this?
        .send()                         // <- connect to host and send request
        .map_err(apperror::AppError::from)    // <- convert SendRequestError to an Error
        .map(
            |resp| {
                let mut new_resp;
                {
                    new_resp = HttpResponseBuilder::from(&resp);
                }
                new_resp.streaming(resp)
            }
        )
        .responder()
}

fn async_forward(req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = AppError>> {
    let mut client_req;
    {
        client_req = client::ClientRequestBuilder::from(&req);
    }
    client_req.uri("https://cameras.liveviewtech.com/users/login")
        .streaming(req).unwrap() // TODO: actually handle this?
        .send()                         // <- connect to host and send request
        .map_err(apperror::AppError::from)    // <- convert SendRequestError to an Error
        .map(
            |resp| {
                let mut new_resp;
                {
                    new_resp = HttpResponseBuilder::from(&resp);
                }
                new_resp.streaming(resp)
            }
        )
        .responder()
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    let _ = env_logger::init();
    let sys = actix::System::new("ws-example");

    server::new(|| {
        App::new()
            // enable logger
            .middleware(middleware::Logger::default())
            .resource("/async", |r| r.route().a(async_forward))
            .resource("/index.html", |r| r.f(|_| "Hello world!"))
            .resource("/", |r| r.f(index))
            .resource("/{catchall:.*}", |r| r.route().a(catchall))
    })
        .threads(4)
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
