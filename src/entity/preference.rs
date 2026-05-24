use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "preference")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: Uuid,
    pub workspace_id: Uuid,
    pub value: Json,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

impl ActiveModelBehavior for ActiveModel {}
