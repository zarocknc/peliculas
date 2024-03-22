//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "Comentarios")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id_comentario: i32,
    pub id_usuario: Option<i32>,
    pub id_pelicula: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub contenido: Option<String>,
    pub fecha_creacion: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::peliculas::Entity",
        from = "Column::IdPelicula",
        to = "super::peliculas::Column::IdPelicula",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Peliculas,
    #[sea_orm(
        belongs_to = "super::usuarios::Entity",
        from = "Column::IdUsuario",
        to = "super::usuarios::Column::IdUsuario",
        on_update = "Restrict",
        on_delete = "Restrict"
    )]
    Usuarios,
}

impl Related<super::peliculas::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Peliculas.def()
    }
}

impl Related<super::usuarios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Usuarios.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}