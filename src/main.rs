#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use std::collections::HashMap;
use tide::{self, http, EndpointResult};
use tide::querystring::ContextExt;

use imagecode::{gen_svg, gen_jpeg, gen_png_buf};

#[cfg(test)]
use http_service_mock::make_server;
#[cfg(test)]
use http_service::Body;
#[cfg(test)]
use futures::executor::block_on;

fn parse_query(cx: tide::Context<()>) -> (String, u32) {
    let query = cx.url_query::<HashMap<String, String>>().unwrap();
    let value = query.get("value").unwrap();
    let size = query.get("size").unwrap_or(&"200".to_string()).parse::<u32>().unwrap();
    (value.to_string(), size)
}

async fn png(cx: tide::Context<()>) -> EndpointResult {
    let (value, size) = parse_query(cx);

    let image = gen_png_buf(value, size);
    let resp = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::PNG.as_ref())
        .header(http::header::CONTENT_DISPOSITION, "inline")
        .status(http::StatusCode::OK)
        .body(image.into())
        .expect("Failed to build response");
    Ok(resp)
}

async fn svg(cx: tide::Context<()>) -> EndpointResult {
    let (value, size) = parse_query(cx);

    let image = gen_svg(value, size);

    let resp = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::TEXT_HTML.as_ref())
        .status(http::StatusCode::OK)
        .body(image.as_bytes().into())
        .expect("Failed to build response");
    Ok(resp)
}

async fn jpeg(cx: tide::Context<()>) -> EndpointResult {
    let (value, size) = parse_query(cx);

    let image = gen_jpeg(value, size);

    let resp = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::JPEG.as_ref())
        .header(http::header::CONTENT_DISPOSITION, "inline")
        .status(http::StatusCode::OK)
        .body(image.into())
        .expect("Failed to build response");
    Ok(resp)
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut app = tide::App::new();

    app.middleware(tide_log::RequestLogger::new());

    app.at("/qr/svg/").get(svg);
    app.at("/qr/png/").get(png);
    app.at("/qr/jpeg/").get(jpeg);
    Ok(app.run("0.0.0.0:8000")?)
}

#[test]
fn png_route_happy_path_no_crash_test() {
    let mut app = tide::App::new();
    app.at("/").get(png);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get(format!("/?value=foo&size=200")).body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();

    block_on(res.into_body().into_vec()).unwrap();
}

#[test]
fn svg_route_happy_path_no_crash_test() {
    let mut app = tide::App::new();
    app.at("/").get(svg);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get(format!("/?value=foo&size=200")).body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();

    block_on(res.into_body().into_vec()).unwrap();
}

#[test]
fn jpeg_route_happy_path_no_crash_test() {
    let mut app = tide::App::new();
    app.at("/").get(jpeg);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http::Request::get(format!("/?value=foo&size=200")).body(Body::empty()).unwrap();
    let res = server.simulate(req).unwrap();

    block_on(res.into_body().into_vec()).unwrap();
}
