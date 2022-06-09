use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
use std::time::Duration;
use actix_web::{get, post, App, HttpRequest, HttpServer, web, HttpResponse};
use serde::Deserialize;

#[derive(Clone)]
struct Counter {
    local: Cell<i32>,
    global: Arc<AtomicI32>,
}

/// 请求链接：
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = Counter {
        // 实现内部可变性，但是因为每次clone都会得到一个新的Cell，所以线程之间是不同的Cell
        local: Cell::new(0),
        // AtomicI32会被多个线程共享，因为对Arc进行clone，得到的Arc彼此之间是共享底层数据的，即共享AtomicI32
        global: Arc::new(AtomicI32::new(0))
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(counter.clone()))
            .route("/t1", web::get().to(
                |counter: web::Data<Counter>| async move {
                    let thread_id = thread::current().id();
                    counter.local.set(counter.local.get() + 1);
                    let local = counter.local.get();
                    counter.global.fetch_add(1, Ordering::Relaxed);
                    let global = counter.global.load(Ordering::Relaxed);
                    println!("{:?}: local: {}, global: {}", thread_id, local, global);
                    // 故意拉慢处理速度，以让新的请求被新的线程处理
                    thread::sleep(Duration::from_secs(1));
                    "ok"
                }
            ))
    })
        .bind("127.0.0.1:8190")?
        .run()
        .await
}