//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "heladera")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Binary(16)")]
    pub uuid: Vec<u8>,
    #[sea_orm(column_type = "Binary(16)")]
    pub direccion_id: Vec<u8>,
    pub cantidad_viandas: u16,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::ubicacion::Entity",        
        from = "Column::DireccionId",
        to = "super::ubicacion::Column::Uuid",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Ubicacion,
}

impl Related<super::ubicacion::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ubicacion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
