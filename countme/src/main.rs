use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};

use actix_web::{web, App, HttpResponse, HttpServer};

async fn index(
    counter: web::Data<AtomicUsize>,
    count: String,
) -> HttpResponse {
    counter.fetch_add(count.parse::<usize>().unwrap_or(0), Ordering::AcqRel);
    HttpResponse::Ok().finish()
}

async fn count(counter: web::Data<AtomicUsize>) -> HttpResponse {
    HttpResponse::Ok().body(counter.load(Ordering::AcqRel).to_string())
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let counter = web::Data::new(AtomicUsize::new(0usize));

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .service(web::resource("/").to(index))
            .service(web::resource("/count").to(count))
    })
        .bind("0.0.0.0:80")?
        .run()
        .await
}
