#![warn(
    rust_2018_idioms,
    future_incompatible
)]

mod endpoints;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut app = tide::new();

    app.middleware(tide::middleware::RequestLogger::new());

    app.at("/qr/svg").get(endpoints::svg);
    app.at("/qr/png").get(endpoints::png);
    app.at("/qr/jpeg").get(endpoints::jpeg);
    Ok(app.listen("0.0.0.0:8000").await?)
}
