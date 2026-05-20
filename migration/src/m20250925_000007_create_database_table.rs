use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Database::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Database::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Database::ConnectionId).integer().not_null())
                    .col(ColumnDef::new(Database::Name).string().not_null())
                    .col(ColumnDef::new(Database::TablesJson).json_binary())
                    .col(ColumnDef::new(Database::ViewsJson).json_binary())
                    .col(ColumnDef::new(Database::MatviewsJson).json_binary())
                    .col(ColumnDef::new(Database::IndexesJson).json_binary())
                    .col(ColumnDef::new(Database::SequencesJson).json_binary())
                    .col(ColumnDef::new(Database::FunctionsJson).json_binary())
                    .col(ColumnDef::new(Database::TriggersJson).json_binary())
                    .col(ColumnDef::new(Database::RulesJson).json_binary())
                    .col(ColumnDef::new(Database::PoliciesJson).json_binary())
                    .col(
                        ColumnDef::new(Database::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Database::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Database::Table, Database::ConnectionId)
                            .to(Connection::Table, Connection::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Database::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Database {
    Table,
    Id,
    ConnectionId,
    Name,
    TablesJson,
    ViewsJson,
    MatviewsJson,
    IndexesJson,
    SequencesJson,
    FunctionsJson,
    TriggersJson,
    RulesJson,
    PoliciesJson,
    CreatedAt,
    UpdatedAt,
}

// Assuming you already have a `connections` table:
#[derive(Iden)]
enum Connection {
    Table,
    Id,
}
