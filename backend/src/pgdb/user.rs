//! SeaORM Entity. Generated by sea-orm-codegen 0.3.2

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uid: i64,
    #[sea_orm(column_type = "Text")]
    pub username: String,
    #[sea_orm(column_type = "Text")]
    pub password: String,
    #[sea_orm(column_type = "Text")]
    pub email: String,
    pub create_time: DateTimeWithTimeZone,
    #[sea_orm(column_type = "Text")]
    pub salt: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            _ => panic!("No RelationDef"),
        }
    }
}

impl Related<super::burrow::Entity> for Entity {
    fn to() -> RelationDef {
        super::favorite::Relation::Burrow.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::favorite::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
