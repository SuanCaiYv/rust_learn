use actix_web::{App, get, post, HttpResponse, HttpServer, Responder, route, web, guard};

async fn get1() -> impl Responder {
    HttpResponse::Ok().body("get1")
}

async fn post1() -> impl Responder {
    HttpResponse::Ok().body("post1")
}

async fn get2() -> impl Responder {
    HttpResponse::Ok().body("get2")
}

async fn post2() -> impl Responder {
    HttpResponse::Ok().body("post2")
}

#[get("/ok")]
async fn ok() -> impl Responder {
    HttpResponse::Ok().body("ok")
}

fn config1(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/t2")
            .route(web::get().to(|| async {
                HttpResponse::Ok().body("get2")
            }))
            // 很明显，这里可以做权限拦截使用，限制请求的方法，或者解析Token等操作
            .route(web::to(|| async {
                HttpResponse::Ok().body("not allowed")
            }))
    );
}

fn config2(config: &mut web::ServiceConfig) {
    config.service(
        web::resource("/t3")
            .route(web::post().to(|| async {
                HttpResponse::Ok().body("get3")
            }))
            .route(web::to(|| async {
                HttpResponse::Ok().body("not allowed")
            }))
    );
}

/// 测试链接：
/// get@/t1
/// get@/t2
/// get@/a
/// !get@/t2
/// get@/ok
/// post@/b/t3
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("/t1")
                    // 要求请求头必须包含如下
                    .guard(guard::Header("Token", "code-with-buff"))
                    .route("", web::get().to(|| async {HttpResponse::Ok().body("ok")}))
            )
            .configure(config1)
            // 产生的请求不会影响到后续的操作，即config不会影响后面的请求路径和方法，是独立的
            .route("/a", web::get().to(|| async {HttpResponse::Ok().body("ok")}))
            // 这里的url会拼接前面的前导url
            .service(web::scope("/b").configure(config2))
            .service(ok)
    })
        .bind("127.0.0.1:8190")?
        .run()
        .await
}
