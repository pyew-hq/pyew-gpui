use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SavedQuery::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(SavedQuery::Id).uuid().primary_key())
                    .col(ColumnDef::new(SavedQuery::ConnectionId).uuid().not_null())
                    .col(ColumnDef::new(SavedQuery::WorkspaceId).integer().not_null())
                    .col(ColumnDef::new(SavedQuery::Title).string().not_null())
                    .col(ColumnDef::new(SavedQuery::QueryText).text().not_null())
                    .col(ColumnDef::new(SavedQuery::Tags).json().null())
                    .col(ColumnDef::new(SavedQuery::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(SavedQuery::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(SavedQuery::Table, SavedQuery::ConnectionId)
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(SavedQuery::Table, SavedQuery::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SavedQuery::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum SavedQuery {
    Table,
    Id,
    ConnectionId,
    WorkspaceId,
    Title,
    QueryText,
    Tags,
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
