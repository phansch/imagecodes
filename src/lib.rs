#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use qrcode::{QrCode, EcLevel};
use qrcode::render::svg;
use image::{ColorType, Luma};
#[cfg(test)]
use pretty_assertions::assert_eq;

pub fn gen_svg(value: String, size: u32) -> String {
    let code = QrCode::with_error_correction_level(value, EcLevel::L).unwrap();

    code.render()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}

pub fn gen_jpeg(value: String, size: u32) -> Vec<u8> {
    let code = QrCode::with_error_correction_level(value, EcLevel::L).unwrap();
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
    let code = QrCode::with_error_correction_level(value, EcLevel::L).expect("Could not encode QR code");
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
    let expected = vec![137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 21, 0, 0, 0, 21, 8, 0, 0, 0, 0, 140, 124, 250, 74, 0, 0, 0, 141, 73, 68, 65, 84, 120, 156, 77, 205, 1, 22, 2, 49, 12, 2, 209, 246, 254, 135, 94, 63, 160, 251, 36, 53, 133, 73, 182, 222, 51, 61, 231, 62, 247, 56, 21, 167, 127, 81, 70, 194, 185, 9, 54, 75, 49, 149, 36, 244, 30, 74, 75, 122, 41, 20, 182, 132, 122, 168, 57, 255, 161, 19, 58, 201, 45, 150, 54, 164, 230, 191, 96, 197, 101, 13, 107, 116, 82, 179, 118, 250, 169, 232, 210, 59, 138, 11, 116, 51, 104, 154, 128, 133, 42, 41, 6, 15, 78, 249, 68, 155, 24, 7, 193, 189, 48, 197, 135, 107, 233, 121, 33, 198, 22, 201, 128, 221, 26, 244, 55, 74, 126, 169, 110, 139, 98, 94, 90, 7, 247, 199, 155, 241, 53, 26, 151, 141, 105, 190, 3, 225, 3, 10, 41, 110, 16, 248, 95, 17, 190, 0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130];
    assert_eq!(gen_png_buf("foo".to_string(), 5), expected);
}

#[test]
fn gen_svg_test() {
    assert_eq!(gen_svg("f".to_string(), 2), include_str!("../tests/test.svg").trim());
}

#[test]
fn long_data_test() {
    let url = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    gen_png_buf(url.to_string(), 5);
}
