use std::io;
use actix_web::{web::{self, Data}, App, HttpResponse, HttpServer};
use std::sync::Mutex;

async fn index(
    numbers: Data<Mutex<Vec<i8>>>,
    count: String,
) -> HttpResponse {
    (*numbers.lock().unwrap())
        .push(count.parse::<i8>().unwrap_or(0));
    HttpResponse::Ok().finish()
}

async fn count(numbers: Data<Mutex<Vec<i8>>>) -> HttpResponse {
    HttpResponse::Ok().body(
        (*numbers.lock().unwrap())
            .iter()
            .map(|num| *num as i32)
            .sum::<i32>()
            .to_string()
    )
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let mut vec = Vec::new();
    vec.reserve(100000);
    let numbers: Data<Mutex<Vec<i8>>> = Data::new(Mutex::new(vec));


    HttpServer::new(move || {
        App::new()
            .app_data(numbers.clone())
            .service(web::resource("/").to(index))
            .service(web::resource("/count").to(count))
    })
        .bind("0.0.0.0:80")?
        .run()
        .await
}
