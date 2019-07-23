#![feature(async_await)]
#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use std::collections::HashMap;
use tide::{self, http, EndpointResult};
use tide::querystring::ContextExt;

use imagecode::{gen_svg, gen_png_buf};

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

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut app = tide::App::new();

    app.middleware(tide_log::RequestLogger::new());

    app.at("/qr/svg/").get(svg);
    app.at("/qr/png/").get(png);
    Ok(app.run("0.0.0.0:8000")?)
}
