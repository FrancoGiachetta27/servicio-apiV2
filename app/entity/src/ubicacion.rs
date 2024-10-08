//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "ubicacion")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Binary(16)")]
    pub uuid: Vec<u8>,
    #[sea_orm(column_type = "Double")]
    pub latitud: f64,
    #[sea_orm(column_type = "Double")]
    pub longitud: f64,
    #[sea_orm(column_type = "Binary(16)")]
    pub direccion_id: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::direccion::Entity",
        from = "Column::DireccionId",
        to = "super::direccion::Column::Uuid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Direccion,
    #[sea_orm(has_many = "super::heladera::Entity")]
    Heladera,
    #[sea_orm(has_many = "super::persona_vulnerable::Entity")]
    PersonaVulnerable,
}

impl Related<super::direccion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Direccion.def()
    }
}

impl Related<super::heladera::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Heladera.def()
    }
}

impl Related<super::persona_vulnerable::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PersonaVulnerable.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
