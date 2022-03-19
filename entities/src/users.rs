//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub uuid: i32,
    pub username: String,
    pub created: i32,
    pub email: String,
    #[sea_orm(column_type = "Custom(\"BLOB\".to_owned())")]
    pub pass_hash: String,
    pub team: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Team
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        todo!()
    }
}

impl ActiveModelBehavior for ActiveModel {}
