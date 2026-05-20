use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QueryHistory::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QueryHistory::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(QueryHistory::ConnectionId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QueryHistory::WorkspaceId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(QueryHistory::QueryText).text().not_null())
                    .col(
                        ColumnDef::new(QueryHistory::ExecutedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QueryHistory::ExecutionTimeMs)
                            .integer()
                            .null(),
                    )
                    .col(ColumnDef::new(QueryHistory::RowsReturned).integer().null())
                    .col(ColumnDef::new(QueryHistory::Status).string().not_null())
                    .col(ColumnDef::new(QueryHistory::ErrorMessage).text().null())
                    .col(ColumnDef::new(QueryHistory::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(QueryHistory::UpdatedAt).timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(QueryHistory::Table, QueryHistory::ConnectionId)
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(QueryHistory::Table, QueryHistory::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QueryHistory::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum QueryHistory {
    Table,
    Id,
    ConnectionId,
    WorkspaceId,
    QueryText,
    ExecutedAt,
    ExecutionTimeMs,
    RowsReturned,
    Status,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
}
