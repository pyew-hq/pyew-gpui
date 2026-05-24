use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "database_object")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub database_id: Uuid,
    pub name: String,
    pub object_type: String,
    pub definition: Option<String>,
    pub metadata: Option<Json>,
    pub created_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
