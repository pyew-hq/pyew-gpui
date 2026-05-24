use sea_orm_migration::prelude::*;

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
            .await
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
