#![feature(async_await)]
#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use qrcode::{QrCode, Version, EcLevel};
use qrcode::render::svg;
use image::{ColorType, Luma};
#[cfg(test)]
use pretty_assertions::{assert_eq};

pub fn gen_svg(value: String, size: u32) -> String {
    let code = QrCode::with_version(value, Version::Normal(3), EcLevel::L).unwrap();

    code.render()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}

pub fn gen_jpeg(value: String, size: u32) -> Vec<u8> {
    let code = QrCode::with_version(value, Version::Normal(3), EcLevel::L).unwrap();
    let image = code.render::<Luma<u8>>()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .build();

    let (width, height) = image.dimensions();
    let mut buf: Vec<u8> = vec![];
    image::jpeg::JPEGEncoder::new(&mut buf)
        .encode(
            &image.into_raw(),
            width,
            height,
            ColorType::Gray(8),
        ).expect("Error on encoding to jpeg");
    buf
}

pub fn gen_png_buf(value: String, size: u32) -> Vec<u8> {
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

#[test]
fn gen_png_buf_test() {
    let expected = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 29, 0,
        0, 0, 29, 8, 0, 0, 0, 0, 115, 248, 56, 211, 0, 0, 0, 230, 73, 68, 65, 84, 120, 156, 93, 207,
        129, 22, 194, 32, 12, 67, 81, 247, 255, 31, 173, 247, 21, 112, 186, 66, 210, 52, 41, 243, 120,
        189, 86, 189, 19, 209, 224, 133, 170, 139, 84, 215, 251, 98, 145, 27, 174, 186, 164, 246, 30,
        89, 52, 238, 157, 178, 124, 103, 227, 153, 202, 233, 3, 115, 46, 229, 213, 249, 210, 129, 195,
        236, 119, 87, 153, 255, 14, 75, 173, 37, 53, 31, 105, 10, 167, 90, 90, 191, 212, 253, 66, 229,
        250, 136, 93, 112, 127, 209, 6, 150, 58, 228, 227, 241, 184, 68, 43, 198, 145, 27, 139, 88, 5,
        214, 26, 173, 221, 216, 51, 54, 204, 134, 105, 163, 55, 94, 233, 45, 112, 244, 104, 3, 57, 162,
        90, 59, 60, 125, 176, 169, 38, 36, 45, 109, 2, 166, 134, 151, 142, 145, 23, 27, 182, 56, 216,
        104, 22, 103, 233, 3, 100, 14, 93, 6, 184, 131, 67, 152, 154, 235, 27, 90, 96, 86, 82, 222,
        148, 231, 147, 0, 191, 25, 210, 85, 122, 40, 120, 181, 14, 37, 99, 118, 180, 100, 55, 248, 191,
        246, 188, 76, 47, 177, 160, 106, 119, 170, 98, 112, 65, 114, 167, 99, 165, 108, 45, 70, 82,
        162, 213, 159, 77, 74, 147, 232, 83, 121, 150, 198, 1, 89, 222, 7, 61, 229, 208, 24, 228, 225,
        188, 29, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
    assert_eq!(gen_png_buf("foo".to_string(), 5), expected);
}

#[test]
fn gen_svg_test() {
    assert_eq!(gen_svg("f".to_string(), 2), include_str!("../tests/test.svg").trim());
}
