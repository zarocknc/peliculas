use dotenv::dotenv;

use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route};
use poem_openapi::OpenApiService;
use sea_orm::Database;
use anyhow::{Context, Result};


mod db;
mod routes;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let url = std::env::var("DATABASE_URL").context("Error al buscar env var DATABASE_URL")?;
    let db = Database::connect(url).await.context("Error al conectar base de datos")?;
    let endpoints = ();
    let api_service = OpenApiService::new(endpoints, "Peliculas API", "0.1.0").server("http://localhost:8080/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service)
    .nest("/", ui)
    .data(db)
    .with(Cors::new());

    poem::Server::new(TcpListener::bind("127.0.0.1:8080")).run(app).await?;
    Ok(())
}