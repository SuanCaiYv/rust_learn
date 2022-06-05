use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

#[tokio::main]
async fn main() {
    let make_server = make_service_fn(|_connection| async {
        Ok::<_, Infallible>(service_fn(global_service))
    });
    let address = SocketAddr::from(([127, 0, 0, 1], 8190));
    let server = Server::bind(&address).serve(make_server);
    if let Err(e) = server.await {
        panic!("{}", e)
    }
}

async fn global_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let service = router(&req);
    let resp = service(&req);
    Ok(*resp)
}

type Service = fn(req: &Request<Body>) -> Box<Response<Body>>;

fn router(req: &Request<Body>) -> Service {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/t1") => {
            fn f(req: &Request<Body>) -> Box<Response<Body>> {
                Box::new(Response::new("ok".into()))
            }
            f
        }
        (&Method::POST, "/t1") => {
            fn f(req: &Request<Body>) -> Box<Response<Body>> {
                // todo!
                Box::new(Response::new("todo".into()))
            }
        }
        _ => {
            fn f(_req: &Request<Body>) -> Box<Response<Body>> {
                Box::new(Response::new("404".into()))
            }
            f
        }
    }
}
