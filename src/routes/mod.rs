use poem_openapi::Tags;

pub mod routes_login;
pub mod routes_peliculas;

#[derive(Tags)]
pub enum ApiTags {
    Auth,
    Peliculas
}