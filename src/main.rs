#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use std::collections::HashMap;
use tide::{self, http, Response};

use imagecode::{gen_svg, gen_jpeg, gen_png_buf};
use http_service::Body;

#[cfg(test)]
use http_service_mock::make_server;
#[cfg(test)]
use async_std::task;
#[cfg(test)]
use async_std::io::ReadExt;

fn parse_query(cx: tide::Request<()>) -> (String, u32) {
    let query = cx.query::<HashMap<String, String>>().unwrap();
    let value = query.get("value").unwrap();
    let size = query.get("size").unwrap_or(&"200".to_string()).parse::<u32>().unwrap();
    (value.to_string(), size)
}

async fn png(cx: tide::Request<()>) -> Response {
    let (value, size) = parse_query(cx);

    let image = gen_png_buf(value, size);
    tide::Response::new(http::StatusCode::OK.into())
        .set_header(http::header::CONTENT_DISPOSITION.as_str(), "inline")
        .body(Body::from(image))
        .set_mime(mime::IMAGE_PNG)
}

async fn svg(cx: tide::Request<()>) -> Response {
    let (value, size) = parse_query(cx);

    let image = gen_svg(value, size);

    tide::Response::new(http::StatusCode::OK.into())
        .set_header(http::header::CONTENT_DISPOSITION.as_str(), "inline")
        .body(Body::from(image.as_bytes().to_vec()))
        .set_mime(mime::IMAGE_SVG)
}

async fn jpeg(cx: tide::Request<()>) -> Response {
    let (value, size) = parse_query(cx);

    let image = gen_jpeg(value, size);

    tide::Response::new(http::StatusCode::OK.into())
        .set_header(http::header::CONTENT_DISPOSITION.as_str(), "inline")
        .body(Body::from(image))
        .set_mime(mime::IMAGE_JPEG)
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut app = tide::new();

    app.middleware(tide::middleware::RequestLogger::new());

    app.at("/qr/svg").get(svg);
    app.at("/qr/png").get(png);
    app.at("/qr/jpeg").get(jpeg);
    Ok(app.listen("0.0.0.0:8000").await?)
}

#[test]
fn png_route_happy_path_no_crash_test() {
    let mut app = tide::new();
    app.at("/").get(png);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get(format!("/?value=foo&size=5")).body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();

    let mut buf = Vec::new();

    assert_eq!(res.headers().get("content-disposition").unwrap(), "inline");
    assert_eq!(res.headers().get("content-type").unwrap(), "image/png");
    task::block_on(res.into_body().read_to_end(&mut buf)).unwrap();
}

#[test]
fn svg_route_happy_path_no_crash_test() {
    let mut app = tide::new();
    app.at("/").get(svg);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get(format!("/?value=foo&size=5")).body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();

    assert_eq!(res.headers().get("content-disposition").unwrap(), "inline");
    assert_eq!(res.headers().get("content-type").unwrap(), "image/svg+xml");
    let mut buf = String::new();
    task::block_on(res.into_body().read_to_string(&mut buf)).unwrap();
}

#[test]
fn jpeg_route_happy_path_no_crash_test() {
    let mut app = tide::new();
    app.at("/").get(jpeg);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get(format!("/?value=foo&size=5")).body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();

    assert_eq!(res.headers().get("content-disposition").unwrap(), "inline");
    assert_eq!(res.headers().get("content-type").unwrap(), "image/jpeg");

    let mut buf = Vec::new();
    task::block_on(res.into_body().read_to_end(&mut buf)).unwrap();
}
