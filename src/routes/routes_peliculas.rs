use poem::web::Data;
use poem_openapi::{OpenApi, payload::Json};
use serde::{Deserialize, Serialize};


use crate::db::entidades::{self, peliculas::Model, prelude::Peliculas};
use sea_orm::{DatabaseConnection, EntityTrait};

use super::ApiTags;


pub struct PeliculasApi;
#[derive(Serialize, Deserialize, Clone, Debug)]
struct PeliculasResponse {
    pub id_pelicula: i32,
    pub titulo: i32,
    pub ano_lanzamiento: i32,
    pub genero: i32,
    pub clasificacion: i32,
    pub id_director: i32,
}

#[OpenApi]
impl PeliculasApi {
    #[oai( path = "/peliculas", method = "get", tag = ApiTags::Peliculas)]
    async fn buscar_pelicula(
        &self,
        data: Data<&DatabaseConnection>
    ) -> Json<Vec<Model>> {
        let pool = data.0.to_owned();
        let pelis =  Peliculas::find().all(data.0).await.unwrap();
        Json(pelis);
        todo!()
    }
}