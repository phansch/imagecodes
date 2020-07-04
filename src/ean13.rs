use barcoders::sym::ean13::*;
use barcoders::generators::image::*;

pub fn gen_png_buf(value: String, size: u32) -> Vec<u8> {
    // TODO: Propagate unwrap error to response once tide has better error handling
    let barcode = EAN13::new(value).unwrap();
    let png = Image::png(size);

    let encoded = barcode.encode();
    png.generate(&encoded[..]).unwrap()
}
