use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Preference::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Preference::Key)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Preference::WorkspaceId).integer().not_null())
                    .col(ColumnDef::new(Preference::Value).json().not_null())
                    .col(ColumnDef::new(Preference::CreatedAt).timestamp().null())
                    .col(ColumnDef::new(Preference::UpdatedAt).timestamp().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Preference::Table, Preference::WorkspaceId)
                            .to(Workspace::Table, Workspace::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Preference::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Preference {
    Table,
    Key,
    WorkspaceId,
    Value,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Workspace {
    Table,
    Id,
}
