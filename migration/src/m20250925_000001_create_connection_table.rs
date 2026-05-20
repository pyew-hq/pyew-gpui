use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Connection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Connection::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Connection::WorkspaceId).integer().not_null())
                    .col(ColumnDef::new(Connection::ConnectionName).string().null())
                    .col(ColumnDef::new(Connection::DatabaseName).string().null())
                    .col(ColumnDef::new(Connection::DbType).string().not_null())
                    .col(ColumnDef::new(Connection::Host).string().null())
                    .col(ColumnDef::new(Connection::Port).integer().null())
                    .col(ColumnDef::new(Connection::Username).string().null())
                    .col(ColumnDef::new(Connection::Password).string().null()) // ⚠️ Store encrypted!
                    .col(ColumnDef::new(Connection::FilePath).string().null())
                    .col(ColumnDef::new(Connection::ExtraParams).json().null())
                    .col(
                        ColumnDef::new(Connection::LastConnectedAt)
                            .timestamp()
                            .null(),
                    )
                    .col(ColumnDef::new(Connection::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(Connection::UpdatedAt).timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Connection::Table, Connection::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Connection::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
    WorkspaceId,
    DatabaseName,
    ConnectionName,
    DbType,
    Host,
    Port,
    Username,
    Password,
    FilePath,
    ExtraParams,
    LastConnectedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
}
