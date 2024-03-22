use anyhow::{Context, Error};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::db::entidades::{peliculas::Model, prelude::Peliculas};

async fn listar_peliculas<'a>(conn: &DatabaseConnection) -> Result<Vec<Model>, Error> {
    let pelis = Peliculas::find().all(conn).await.context("Error al buscar peliculas")?;
    Ok(pelis)
}
