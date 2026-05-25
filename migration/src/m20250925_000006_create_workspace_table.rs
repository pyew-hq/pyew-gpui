use sea_orm_migration::prelude::*;

const PRIMARY_WORKSPACE_ID_HEX: &str = "00000000000040008000000000000001";
const PRIMARY_WORKSPACE_NAME: &str = "Primary";

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Workspace::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Workspace::Id).uuid().primary_key())
                    .col(ColumnDef::new(Workspace::Name).string().not_null())
                    .col(ColumnDef::new(Workspace::IsOpened).boolean().null())
                    .col(ColumnDef::new(Workspace::LastOpened).timestamp().null())
                    .col(ColumnDef::new(Workspace::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(Workspace::UpdatedAt).timestamp().null())
                    .to_owned(),
            )
            .await?;

        let now = crate::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let insert_primary_workspace = format!(
            "INSERT INTO workspace (id, name, is_opened, last_opened, created_at, updated_at) \
             SELECT x'{PRIMARY_WORKSPACE_ID_HEX}', '{PRIMARY_WORKSPACE_NAME}', 1, '{now}', '{now}', '{now}' \
             WHERE NOT EXISTS (SELECT 1 FROM workspace WHERE name = '{PRIMARY_WORKSPACE_NAME}')"
        );

        manager
            .get_connection()
            .execute_unprepared(&insert_primary_workspace)
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Workspace::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
    Name,
    IsOpened,
    LastOpened,
    CreatedAt,
    UpdatedAt,
}
