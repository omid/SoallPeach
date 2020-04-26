use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};

use actix_web::{web, App, HttpResponse, HttpServer};

async fn index(
    counter: web::Data<AtomicUsize>,
    count: String,
) -> HttpResponse {
    let count = count.parse::<usize>().unwrap();
    counter.fetch_add(count, Ordering::SeqCst);
    HttpResponse::Ok().finish()
}

async fn count(counter: web::Data<AtomicUsize>) -> HttpResponse {
    HttpResponse::Ok().body(counter.load(Ordering::SeqCst).to_string())
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
