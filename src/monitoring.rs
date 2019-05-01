use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::rt::{self, Future};
use hyper::{Body, Request, Response, Server, StatusCode};
use hyper_router::{Route, RouterBuilder, RouterService};
use prometheus::{Encoder, TextEncoder};
use std::net::SocketAddr;
use std::string::FromUtf8Error;
use serde::{Deserialize};

fn alive_check(_: Request<Body>) -> Response<Body> {
    let body = "alive and running";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn health_check(_: Request<Body>) -> Response<Body> {
    let body = "This is the default healthcheck";
    Response::builder()
        .header(CONTENT_LENGTH, body.len() as u64)
        .header(CONTENT_TYPE, "text/plain")
        .body(Body::from(body))
        .expect("Failed to construct the response")
}

fn metric_request(_: Request<Body>) -> Response<Body> {
    let mut response = Response::builder();
    match process_metrics() {
        Ok(body) => response
            .header(CONTENT_LENGTH, body.len() as u64)
            .header(CONTENT_TYPE, "text/plain")
            .body(Body::from(body))
            .expect("Failed to construct the response"),

        Err(_) => response
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .expect("Failed to construct the response"),
    }
}

fn process_metrics() -> Result<String, FromUtf8Error> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer)
}

fn build_router_service() -> Result<RouterService, std::io::Error> {
    let router = RouterBuilder::new()
        .add(Route::get("/alive.txt").using(alive_check))
        .add(Route::get("/healthckeck").using(health_check))
        .add(Route::get("/prometheus/metrics").using(metric_request))
        .build();
    Ok(RouterService::new(router))
}

#[derive(Debug, Deserialize)]
pub struct MonitoringConfig{
    enabled: bool,
    bind_addr: String,
    bind_port: String,
}

impl Default for MonitoringConfig{
    fn default() -> Self { 
        MonitoringConfig {
            enabled: true,
            bind_addr: "0.0.0.0".to_string(),
            bind_port: "9080".to_string(),
        }
    }
}

pub fn start_beacon<'a>(config: &MonitoringConfig) {
    info!("Starting monitoring beacon for this app");
    if !config.enabled {
        warn!("The http beacon for health and metrics is disabled");
        return;
    }
    let bind_addr = format!("{}:{}", config.bind_addr, config.bind_port);
    let addr: SocketAddr = bind_addr.parse()
                                    .expect("Inavalid socket address, cannot start server");
    info!("Attempting to start beacon server at {}", bind_addr);
    rt::run(rt::lazy(move || {
        let server = Server::bind(&addr)
            .serve(build_router_service)
            .map_err(|e| error!("server error: {}", e));

        rt::spawn(server);
        Ok(())
    }));
}

