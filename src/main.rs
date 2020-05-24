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

    app.at("/qr/svg").get(endpoints::svg);
    app.at("/qr/png").get(endpoints::png);
    app.at("/qr/jpeg").get(endpoints::jpeg);
    log::info!("App is listening");
    Ok(app.listen("0.0.0.0:8000").await?)
}
