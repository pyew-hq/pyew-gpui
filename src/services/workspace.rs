use crate::entity::workspace;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbConn, DbErr, EntityTrait,
    QueryFilter, QueryOrder,
};
use uuid::Uuid;

const PRIMARY_WORKSPACE_ID: &str = "00000000-0000-4000-8000-000000000001";
const PRIMARY_WORKSPACE_ID_HEX: &str = "00000000000040008000000000000001";
const PRIMARY_WORKSPACE_NAME: &str = "Primary";

#[allow(dead_code)]
pub struct WorkspaceService;

#[allow(dead_code)]
impl WorkspaceService {
    pub async fn get_or_create_opened_workspace(db: &DbConn) -> Result<workspace::Model, DbErr> {
        Self::repair_primary_workspace_id(db).await?;

        if let Some(workspace) = workspace::Entity::find()
            .filter(workspace::Column::IsOpened.eq(true))
            .order_by_desc(workspace::Column::LastOpened)
            .one(db)
            .await?
        {
            return Ok(workspace);
        }

        if let Some(workspace) = workspace::Entity::find()
            .filter(workspace::Column::Name.eq(PRIMARY_WORKSPACE_NAME))
            .one(db)
            .await?
        {
            return Self::mark_workspace_opened(db, workspace).await;
        }

        let now = migration::now();
        let workspace_id = Uuid::parse_str(PRIMARY_WORKSPACE_ID)
            .map_err(|err| DbErr::Custom(format!("invalid primary workspace id: {err}")))?;

        workspace::ActiveModel {
            id: Set(workspace_id),
            name: Set(PRIMARY_WORKSPACE_NAME.to_string()),
            is_opened: Set(Some(true)),
            last_opened: Set(Some(now)),
            created_at: Set(Some(now)),
            updated_at: Set(Some(now)),
        }
        .insert(db)
        .await
    }

    async fn mark_workspace_opened(
        db: &DbConn,
        workspace: workspace::Model,
    ) -> Result<workspace::Model, DbErr> {
        let now = migration::now();
        let mut workspace: workspace::ActiveModel = workspace.into();
        workspace.is_opened = Set(Some(true));
        workspace.last_opened = Set(Some(now));
        workspace.updated_at = Set(Some(now));
        workspace.update(db).await
    }

    async fn repair_primary_workspace_id(db: &DbConn) -> Result<(), DbErr> {
        let repair_primary_workspace = format!(
            "UPDATE workspace \
             SET id = x'{PRIMARY_WORKSPACE_ID_HEX}' \
             WHERE name = '{PRIMARY_WORKSPACE_NAME}' AND typeof(id) = 'text'"
        );

        db.execute_unprepared(&repair_primary_workspace)
            .await
            .map(|_| ())
    }
}
