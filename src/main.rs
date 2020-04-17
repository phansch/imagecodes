#![warn(
    rust_2018_idioms,
    future_incompatible
)]

use tide::Request;

mod endpoints;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut app = tide::new();

    app.middleware(tide::middleware::RequestLogger::new());

    app.at("/qr/svg").get(endpoints::svg);
    app.at("/qr/png").get(endpoints::png);
    app.at("/qr/jpeg").get(endpoints::jpeg);
    app.at("/qr/:qr_type").get(|req: Request<()>| async move {
        let qr_type = &req.param("qr_type").unwrap_or(String::new());
        match qr_type.as_str() {
            "svg" => endpoints::svg(req).await,
            "png" => endpoints::png(req).await,
            "jpeg" => endpoints::jpeg(req).await,
            _ => endpoints::qr_type_not_supported(req).await
        }
    });
    Ok(app.listen("0.0.0.0:8000").await?)
}
