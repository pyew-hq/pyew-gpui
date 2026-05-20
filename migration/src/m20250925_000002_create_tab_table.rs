use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tab::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Tab::Id)
                            .integer()
                            .auto_increment()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Tab::ConnectionId).integer().not_null())
                    .col(ColumnDef::new(Tab::WorkspaceId).integer().not_null())
                    .col(ColumnDef::new(Tab::Title).string().not_null())
                    .col(ColumnDef::new(Tab::QueryText).text().not_null())
                    .col(ColumnDef::new(Tab::CursorPosition).integer().default(0))
                    .col(ColumnDef::new(Tab::IsPinned).boolean().default(false))
                    .col(ColumnDef::new(Tab::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Tab::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tab::Table, Tab::ConnectionId)
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Tab::Table, Tab::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tab::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Tab {
    Table,
    Id,
    ConnectionId,
    WorkspaceId,
    Title,
    QueryText,
    CursorPosition,
    IsPinned,
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
