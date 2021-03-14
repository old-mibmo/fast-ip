use std::{
    convert::Infallible,
    include_str,
    net::{IpAddr, SocketAddr},
};

use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

const BIND_ADDR: ([u8; 4], u16) = ([0, 0, 0, 0], 3000);

async fn landing_page() -> Result<Response<Body>, Infallible> {
    let body = include_str!("../res/landing.html");

    let resp = Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(body.into())
        .expect("failed to build response");

    Ok(resp)
}

async fn error_404(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body = format!(
        "You've hit a 404 error; the page at {path} doesn't exist",
        path = req.uri().path()
    );

    let resp = Response::builder()
        .status(404)
        .header("Content-Type", "text/plain")
        .body(body.into())
        .expect("failed to build response");

    Ok(resp)
}

async fn ip_plain(addr: IpAddr) -> Result<Response<Body>, Infallible> {
    let body = addr.to_string();

    let resp = Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(body.into())
        .expect("failed to build response");

    Ok(resp)
}

async fn ip_json(addr: IpAddr) -> Result<Response<Body>, Infallible> {
    let body = format!("{{\"ip\":\"{ip}\"}}", ip = addr);

    let resp = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(body.into())
        .expect("failed to build response");

    Ok(resp)
}

async fn mux(req: Request<Body>, addr: IpAddr) -> Result<Response<Body>, Infallible> {
    match req.uri().path() {
        "/" | "/ip" | "/plain" => ip_plain(addr).await,
        "/json" => ip_json(addr).await,
        "/what" => landing_page().await,
        _ => error_404(req).await,
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(BIND_ADDR);

    let make_service = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr().ip();
        async move { Ok::<_, Infallible>(service_fn(move |req| mux(req, addr.clone()))) }
    });
    let server = Server::bind(&addr).serve(make_service);

    println!("listening on {}", addr,);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
