use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "config", rename_all = "snake_case")]
pub enum ConnectionConfig {
    Postgres(PostgresConfig),
    #[serde(rename = "mysql")]
    MySql(MySqlConfig),
    Sqlite(SqliteConfig),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: Option<String>,
    pub ssl_mode: Option<String>,
    pub extra_params: Option<Json>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: Option<String>,
    pub extra_params: Option<Json>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SqliteConfig {
    pub file_path: String,
    pub extra_params: Option<Json>,
}

#[allow(dead_code)]
impl ConnectionConfig {
    pub fn database_type(&self) -> &'static str {
        match self {
            Self::Postgres(_) => "postgres",
            Self::MySql(_) => "mysql",
            Self::Sqlite(_) => "sqlite",
        }
    }

    pub fn into_json(self) -> Result<Json, serde_json::Error> {
        serde_json::to_value(self)
    }

    pub fn from_json(value: Json) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "connection")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(
        belongs_to = "super::workspace::Entity",
        from = "Column::WorkspaceId",
        to = "super::workspace::Column::Id"
    )]
    pub workspace_id: Uuid,

    pub connection_name: Option<String>,
    pub connection_config: Json,
    pub last_connected_at: Option<DateTime>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
}

#[allow(dead_code)]
impl Model {
    pub fn config(&self) -> Result<ConnectionConfig, serde_json::Error> {
        ConnectionConfig::from_json(self.connection_config.clone())
    }
}

impl ActiveModelBehavior for ActiveModel {}
