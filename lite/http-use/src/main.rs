use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use hyper::{Body, Error, Method, Request, Response, Server};
use hyper::body::to_bytes;
use hyper::service::{make_service_fn, service_fn};
use url::form_urlencoded::parse;

#[tokio::main]
async fn main() {
    let address = SocketAddr::from(([127, 0, 0, 1], 8190));
    let server = Server::bind(&address).serve(make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(router))
    }));
    if let Err(e) = server.await {
        panic!("{}", e)
    }
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/t1") => {
            Ok(get(req).await)
        }
        (&Method::POST, "/t1") => {
            Ok(post(req).await)
        }
        _ => {
            Ok(not_found(req).await)
        }
    }
}
async fn not_found(_req: Request<Body>) -> Response<Body> {
    Response::builder().status(404).body(Body::empty()).unwrap()
}

async fn post(req: Request<Body>) -> Response<Body> {
    // 这里涉及到字段所有权，所以没法传req的引用
    let bytes = to_bytes(req).await.unwrap();
    // 尝试解析请求参数
    let params = parse(bytes.as_ref()).into_owned().collect::<HashMap<String, String>>();
    println!("{:?}", params);
    Response::new(Body::from("ok".to_string()))
}

async fn get(req: Request<Body>) -> Response<Body> {
    let queries = req.uri().query().unwrap();
    let params = parse(queries.as_bytes()).into_owned().collect::<HashMap<String, String>>();
    println!("{:?}", params);
    Response::new(Body::from("ok".to_string()))
}
