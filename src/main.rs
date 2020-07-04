#![warn(
    rust_2018_idioms,
    future_incompatible
)]

mod endpoints;
use tide::log;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    log::start();

    let mut app = tide::new();

    app.at("/qr/svg").get(endpoints::qr::svg);
    app.at("/qr/png").get(endpoints::qr::png);
    app.at("/qr/jpeg").get(endpoints::qr::jpeg);
    app.at("/ean13/png").get(endpoints::ean13::png);
    app.at("/debug").get(endpoints::debug);
    log::info!("App is listening");
    Ok(app.listen("0.0.0.0:8000").await?)
}
