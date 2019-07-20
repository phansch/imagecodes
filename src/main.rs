#![feature(async_await)]
#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use qrcode::{QrCode, Version, EcLevel};
use qrcode::render::svg;
use image::{ColorType, Luma};

use tide::{self, http, EndpointResult};
use tide::querystring::ContextExt;
use std::collections::HashMap;

// TODO: share query parsing
// TODO: Abstract from web server as much as possible
// What would be needed to launch a product that has chances of success?
// * API docs
// * Nice website
// * SSL
// * Rate Limiting
// * PNG generation
// * UI

// MVP:
// No nice website
// No UI
// No API docs
// No Domain
// Yes PNG generation
// Yes SVG generation


fn gen_svg(value: String, size: u32) -> String {
    let code = QrCode::with_version(value, Version::Normal(3), EcLevel::L).unwrap();

    code.render()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}

fn gen_png_buf(value: String, size: u32) -> Vec<u8> {
    let code = QrCode::with_version(value, Version::Normal(3), EcLevel::L).unwrap();
    let image = code.render::<Luma<u8>>()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .build();

    let (width, height) = image.dimensions();

    let mut buf: Vec<u8> = vec![];
    image::png::PNGEncoder::new(&mut buf)
        .encode(
            &image.into_raw(),
            width,
            height,
            ColorType::Gray(8),
        ).expect("Error on encoding to png");
    buf
}

async fn png(cx: tide::Context<()>) -> EndpointResult {
    let query = cx.url_query::<HashMap<String, String>>()?;
    let value = query.get("value").unwrap();
    let size = query.get("size").unwrap_or(&"200".to_string()).parse::<u32>().unwrap();

    let image = gen_png_buf(value.to_string(), size);
    let resp = http::Response::builder()
        .header(http::header::CONTENT_TYPE, mime::PNG.as_ref())
        .header(http::header::CONTENT_DISPOSITION, "inline")
        .status(http::StatusCode::OK)
        .body(image.into())
        .expect("Failed to build response");
    Ok(resp)
}

async fn svg(cx: tide::Context<()>) -> EndpointResult {
    let query = cx.url_query::<HashMap<String, String>>()?;
    let value = query.get("value").unwrap();
    let size = query.get("size").unwrap_or(&"200".to_string()).parse::<u32>().unwrap();

    let image = gen_svg(value.to_string(), size);

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
    Ok(app.run("127.0.0.1:8000")?)
}
