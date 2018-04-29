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

mod apperror;

use apperror::*;

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn catchall(s: Path<String>) -> Box<Future<Item = HttpResponse, Error = AppError>> {
    println!("s => {}", s.clone());
    let base_url = "https://cameras.liveviewtech.com/".to_owned();
    let full_url = format!("{}{}", base_url, s.into_inner());
    client::ClientRequest::get(&full_url)
        .finish().unwrap()
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

fn async_forward(_req: HttpRequest) -> Box<Future<Item = HttpResponse, Error = AppError>> {
    client::ClientRequest::get("https://cameras.liveviewtech.com/users/login") //https://www.rust-lang.org/en-US/  //http://liveviewtech.com/ //https://www.google.com/
        .finish().unwrap()
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
            .resource("/{catchall:.*}", |r| r.with(catchall))
    })
        .threads(4)
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    println!("Started http server: 127.0.0.1:8080");
    let _ = sys.run();
}
