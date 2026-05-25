use crate::entity::connection::{self, ConnectionConfig};
use sea_orm::entity::prelude::{DateTime, Uuid};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter,
    QueryOrder,
};

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CreateConnection {
    pub workspace_id: Uuid,
    pub connection_name: Option<String>,
    pub connection_config: ConnectionConfig,
    pub last_connected_at: Option<DateTime>,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct UpdateConnection {
    pub connection_name: Option<Option<String>>,
    pub connection_config: Option<ConnectionConfig>,
    pub last_connected_at: Option<Option<DateTime>>,
}

#[allow(dead_code)]
pub struct ConnectionService;

#[allow(dead_code)]
impl ConnectionService {
    pub async fn create_connection(
        db: &DbConn,
        input: CreateConnection,
    ) -> Result<connection::Model, DbErr> {
        let now = migration::now();

        connection::ActiveModel {
            id: Set(Uuid::new_v4()),
            workspace_id: Set(input.workspace_id),
            connection_name: Set(input.connection_name),
            connection_config: Set(connection_config_to_json(input.connection_config)?),
            last_connected_at: Set(input.last_connected_at),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        }
        .insert(db)
        .await
    }

    pub async fn update_connection(
        db: &DbConn,
        id: Uuid,
        input: UpdateConnection,
    ) -> Result<Option<connection::Model>, DbErr> {
        let Some(connection) = connection::Entity::find_by_id(id).one(db).await? else {
            return Ok(None);
        };

        let mut connection: connection::ActiveModel = connection.into();

        if let Some(connection_name) = input.connection_name {
            connection.connection_name = Set(connection_name);
        }

        if let Some(connection_config) = input.connection_config {
            connection.connection_config = Set(connection_config_to_json(connection_config)?);
        }

        if let Some(last_connected_at) = input.last_connected_at {
            connection.last_connected_at = Set(last_connected_at);
        }

        connection.updated_at = Set(Some(migration::now()));

        connection.update(db).await.map(Some)
    }

    pub async fn delete_connection(db: &DbConn, id: Uuid) -> Result<bool, DbErr> {
        let result = connection::Entity::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected > 0)
    }

    pub async fn fetch_connection_by_id(
        db: &DbConn,
        id: Uuid,
    ) -> Result<Option<connection::Model>, DbErr> {
        connection::Entity::find_by_id(id).one(db).await
    }

    pub async fn fetch_connections_by_workspace_id(
        db: &DbConn,
        workspace_id: Uuid,
    ) -> Result<Vec<connection::Model>, DbErr> {
        connection::Entity::find()
            .filter(connection::Column::WorkspaceId.eq(workspace_id))
            .order_by_asc(connection::Column::ConnectionName)
            .order_by_asc(connection::Column::CreatedAt)
            .all(db)
            .await
    }
}

fn connection_config_to_json(config: ConnectionConfig) -> Result<sea_orm::JsonValue, DbErr> {
    config
        .into_json()
        .map_err(|err| DbErr::Custom(format!("failed to serialize connection config: {err}")))
}
