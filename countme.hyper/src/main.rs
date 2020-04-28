use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Error, Response, Server, Method, StatusCode};
use std::str;

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 80).into();

    let counter = Arc::new(AtomicUsize::new(0));

    let make_service = make_service_fn(move |_| {
        let counter = counter.clone();

        async move {
            Ok::<_, Error>(service_fn(move |req| {
                let counter = counter.clone();

                async move {
                    match (req.method(), req.uri().path()) {
                        (&Method::GET, "/count") => Ok::<_, Error>(Response::new(Body::from(counter.load(Ordering::Acquire).to_string()))),
                        (&Method::POST, "/") => {
                            let num = hyper::body::to_bytes(req).await?;
                            let num = str::from_utf8(&num).map(str::to_owned).unwrap();

                            counter.fetch_add(num.parse::<usize>().unwrap_or(0), Ordering::AcqRel);

                            return Ok::<_, Error>(Response::builder()
                                .body(Body::empty())
                                .unwrap());
                        }
                        _ => Ok::<_, Error>(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::empty())
                            .unwrap()),
                    }
                }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
