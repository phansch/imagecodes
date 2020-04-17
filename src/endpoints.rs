use std::collections::HashMap;
use imagecodes::{gen_svg, gen_jpeg, gen_png_buf};
use tide::Response;
use http_types::headers::HeaderName;
use http_types::StatusCode;
use proptest::prelude::*;
use std::str::FromStr;
use async_std::io::Cursor;

#[cfg(test)]
use http_service_mock::make_server;
#[cfg(test)]
use async_std::task;
#[cfg(test)]
use async_std::io::ReadExt;
#[cfg(test)]
use http_types::headers;

fn content_disposition() -> HeaderName {
    HeaderName::from_str("content-disposition").unwrap()
}

fn parse_query(cx: tide::Request<()>) -> (String, u32) {
    let query = cx.query::<HashMap<String, String>>().unwrap();
    let value = query.get("value").unwrap();
    let size = query.get("size").unwrap_or(&"200".to_string()).parse::<u32>().unwrap();
    (value.to_string(), size)
}


pub async fn svg(cx: tide::Request<()>) -> Response {
    let (value, size) = parse_query(cx);

    let image = gen_svg(value, size);

    tide::Response::new(StatusCode::Ok)
        .set_header(content_disposition(), "inline")
        .body_string(image)
        .set_mime(mime::IMAGE_SVG)
}

pub async fn jpeg(cx: tide::Request<()>) -> Response {
    let (value, size) = parse_query(cx);

    let image = gen_jpeg(value, size);
    tide::Response::new(StatusCode::Ok)
        .set_header(content_disposition(), "inline")
        .body(Cursor::new(image))
        .set_mime(mime::IMAGE_JPEG)
}

pub async fn png(cx: tide::Request<()>) -> Response {
    let (value, size) = parse_query(cx);

    let image = gen_png_buf(value, size);
    tide::Response::new(StatusCode::Ok)
        .set_header(content_disposition(), "inline")
        .body(Cursor::new(image))
        .set_mime(mime::IMAGE_PNG)
}

#[test]
fn png_route_happy_path_no_crash_test() {
    let mut app = tide::new();
    app.at("/").get(png);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http_types::Request::new(http_types::Method::Get, format!("https://example.com/?value=foo&size=5").parse().unwrap());
    let mut res = server.simulate(req).unwrap();

    let mut buf = Vec::new();

    assert_eq!(res.header(&content_disposition()).unwrap()[0], "inline");
    assert_eq!(res.header(&headers::CONTENT_TYPE).unwrap()[0], "image/png");
    task::block_on(res.read_to_end(&mut buf)).unwrap();
}

#[test]
fn svg_route_happy_path_no_crash_test() {
    let mut app = tide::new();
    app.at("/").get(svg);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http_types::Request::new(http_types::Method::Get, format!("https://example.com/?value=foo&size=5").parse().unwrap());
    let mut res = server.simulate(req).unwrap();

    assert_eq!(res.header(&content_disposition()).unwrap()[0], "inline");
    assert_eq!(res.header(&headers::CONTENT_TYPE).unwrap()[0], "image/svg+xml");
    let mut buf = String::new();
    task::block_on(res.read_to_string(&mut buf)).unwrap();
}

#[test]
fn jpeg_route_happy_path_no_crash_test() {
    let mut app = tide::new();
    app.at("/").get(jpeg);
    let mut server = make_server(app.into_http_service()).unwrap();

    let req = http_types::Request::new(http_types::Method::Get, format!("https://example.com?value=foo&size=5").parse().unwrap());
    let mut res = server.simulate(req).unwrap();

    assert_eq!(res.header(&content_disposition()).unwrap()[0], "inline");
    assert_eq!(res.header(&headers::CONTENT_TYPE).unwrap()[0], "image/jpeg");

    let mut buf = Vec::new();
    task::block_on(res.read_to_end(&mut buf)).unwrap();
}

proptest! {
    #[test]
    fn jpeg_route(s in "[a-zA-Z0-9]*") {
        let mut app = tide::new();
        app.at("/").get(jpeg);
        let mut server = make_server(app.into_http_service()).unwrap();

        let req = http_types::Request::new(http_types::Method::Get, format!("https://example.com?value={}&size=5", s).parse().unwrap());
        let mut res = server.simulate(req)?;
        assert_eq!(res.header(&content_disposition()).unwrap()[0], "inline");
        assert_eq!(res.header(&headers::CONTENT_TYPE).unwrap()[0], "image/jpeg");
        let mut buf = Vec::new();
        task::block_on(res.read_to_end(&mut buf)).unwrap();
    }
}
