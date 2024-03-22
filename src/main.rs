use std::io::Error;
use dotenv::dotenv;

use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use poem_openapi::OpenApiService;

mod db;


#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    let endpoints = ();
    let api_service = OpenApiService::new(endpoints, "Peliculas API", "0.1.0").server("http://localhost:8080/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service)
    .nest("/", ui)
    .with(Cors::new());
    poem::Server::new(TcpListener::bind("127.0.0.1:8080")).run(app).await
}
