#![feature(async_await)]
#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use qrcode::{QrCode, Version, EcLevel};
use qrcode::render::svg;
use image::{ColorType, Luma};

pub fn gen_svg(value: String, size: u32) -> String {
    let code = QrCode::with_version(value, Version::Normal(3), EcLevel::L).unwrap();

    code.render()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}

pub fn gen_png_buf(value: String, size: u32) -> Vec<u8> {
    let code = QrCode::with_version(value, Version::Normal(3), EcLevel::L).unwrap();
    let image = code.render::<Luma<u8>>()
        .min_dimensions(size, size)
        .quiet_zone(false)
        .build();

    let mut buf: Vec<u8> = vec![];
    image::png::PNGEncoder::new(&mut buf)
        .encode(
            &image.into_raw(),
            size,
            size,
            ColorType::Gray(8),
        ).expect("Error on encoding to png");
    buf
}


