use std::borrow::BorrowMut;
use std::sync::Mutex;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};

#[get("/t1")]
async fn get_request() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

async fn post_request(body: String) -> impl Responder {
    println!("{}", body);
    HttpResponse::Ok().body("ok")
}

#[get("/t2")]
async fn app_data_test(data: web::Data<Mutex<i32>>) -> impl Responder {
    let mut counter = data.lock().unwrap();
    *counter += 1;
    println!("counter is: {}", counter);
    HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(Mutex::new(0));
    HttpServer::new(move || {
        App::new()
            .service(get_request)
            .route("/t1", web::post().to(post_request))
            .app_data(counter.clone())
            .service(app_data_test)
    })
        .bind("127.0.0.1:8190")?
        .run()
        .await
}
