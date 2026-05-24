use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "saved_query")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub connection_id: Uuid,
    pub workspace_id: Uuid,
    pub title: String,
    pub query_text: String,
    pub tags: Option<Json>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ActiveModelBehavior for ActiveModel {}
