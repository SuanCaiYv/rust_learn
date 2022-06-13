use actix_web::{get, HttpServer, App};
use actix_web::middleware::{DefaultHeaders, Logger};
use env_logger::Env;

#[get("/t1")]
async fn f() -> String {
    "ok".to_string()
}

/// 请求连接
/// post@/t1
/// post@/t2
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new().add(("aaa", "bbb")))
            .wrap(DefaultHeaders::new().add(("aaa", "aaa")))
            .wrap(Logger::new("%a"))
            .wrap(Logger::new("%t"))
            .service(f)
    })
        .bind(("127.0.0.1", 8190))?
        .run()
        .await
}