use sea_orm::entity::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "attendance")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: String,
    pub clock_in_time: NaiveDateTime,
    pub clock_out_time: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
