use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "query_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub connection_id: Uuid,
    pub workspace_id: Uuid,
    pub query_text: String,
    pub executed_at: DateTime,
    pub execution_time_ms: Option<i32>,
    pub rows_returned: Option<i32>,
    pub status: String,
    pub error_message: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl ActiveModelBehavior for ActiveModel {}
