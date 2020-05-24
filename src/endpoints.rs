use std::collections::HashMap;
use imagecodes::{gen_svg, gen_jpeg, gen_png_buf};
use tide::{Response, StatusCode};
use proptest::prelude::*;
use async_std::io::Cursor;

#[cfg(test)]
use http_types::url::Url;
#[cfg(test)]
use async_std::io::ReadExt;
#[cfg(test)]
use http_types::headers;
#[cfg(test)]
use tide::Server;

static CONTENT_DISPOSITION: &str = "content-disposition";

fn parse_query(cx: tide::Request<()>) -> (String, u32) {
    let query = cx.query::<HashMap<String, String>>().unwrap();
    let value = query.get("value").unwrap();
    let size = query.get("size").unwrap_or(&"200".to_string()).parse::<u32>().unwrap();
    (value.to_string(), size)
}


pub async fn svg(cx: tide::Request<()>) -> tide::Result {
    let (value, size) = parse_query(cx);

    let image = gen_svg(value, size);

    Ok(Response::new(StatusCode::Ok)
        .set_header(CONTENT_DISPOSITION, "inline")
        .body_string(image)
        .set_mime(mime::IMAGE_SVG))
}

pub async fn jpeg(cx: tide::Request<()>) -> tide::Result {
    let (value, size) = parse_query(cx);

    let image = gen_jpeg(value, size);
    Ok(Response::new(StatusCode::Ok)
        .set_header(CONTENT_DISPOSITION, "inline")
        .body(Cursor::new(image))
        .set_mime(mime::IMAGE_JPEG))
}

pub async fn png(cx: tide::Request<()>) -> tide::Result {
    let (value, size) = parse_query(cx);

    let image = gen_png_buf(value, size);
    Ok(Response::new(StatusCode::Ok)
        .set_header(CONTENT_DISPOSITION, "inline")
        .body(Cursor::new(image))
        .set_mime(mime::IMAGE_PNG))
}

#[async_std::test]
async fn png_route_happy_path_no_crash_test() {
    let mut app = Server::new();
    app.at("/").get(png);

    let req = http_types::Request::new(http_types::Method::Get, Url::parse("https://example.com/?value=foo&size=5").unwrap());
    let mut res: tide::http::Response = app.respond(req).await.unwrap();

    let mut buf = Vec::new();

    assert_eq!(res.header(CONTENT_DISPOSITION).unwrap()[0], "inline");
    assert_eq!(res.header(headers::CONTENT_TYPE.as_str()).unwrap()[0], "image/png");
    res.read_to_end(&mut buf);
}

#[async_std::test]
async fn svg_route_happy_path_no_crash_test() {
    let mut app = Server::new();
    app.at("/").get(svg);

    let req = http_types::Request::new(http_types::Method::Get, Url::parse("https://example.com/?value=foo&size=5").unwrap());
    let mut res: tide::http::Response = app.respond(req).await.unwrap();

    assert_eq!(res.header(CONTENT_DISPOSITION).unwrap()[0], "inline");
    assert_eq!(res.header(headers::CONTENT_TYPE.as_str()).unwrap()[0], "image/svg+xml");
    let mut buf = String::new();
    res.read_to_string(&mut buf);
}

#[async_std::test]
async fn jpeg_route_happy_path_no_crash_test() {
    let mut app = Server::new();
    app.at("/").get(jpeg);

    let req = http_types::Request::new(http_types::Method::Get, Url::parse("https://example.com?value=foo&size=5").unwrap());
    let mut res: tide::http::Response = app.respond(req).await.unwrap();

    assert_eq!(res.header(CONTENT_DISPOSITION).unwrap()[0], "inline");
    assert_eq!(res.header(headers::CONTENT_TYPE.as_str()).unwrap()[0], "image/jpeg");

    let mut buf = Vec::new();
    res.read_to_end(&mut buf);
}

proptest! {
    #[test]
    #[allow(unused_must_use)]
    fn jpeg_route(s in "[a-zA-Z0-9]*") {
        let mut app = Server::new();
        app.at("/").get(jpeg);

        let req = http_types::Request::new(http_types::Method::Get, Url::parse(&format!("https://example.com?value={}&size=5", s)).unwrap());
        async {
            let mut res: tide::http::Response = app.respond(req).await.unwrap();
            assert_eq!(res.header(CONTENT_DISPOSITION).unwrap()[0], "inline");
            assert_eq!(res.header(headers::CONTENT_TYPE.as_str()).unwrap()[0], "image/jpeg");
            let mut buf = Vec::new();
            res.read_to_end(&mut buf);
        };
    }
}
