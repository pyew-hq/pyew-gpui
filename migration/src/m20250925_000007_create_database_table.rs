use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Create the Database table (High-level info)
        manager
            .create_table(
                Table::create()
                    .table(Database::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Database::Id).uuid().primary_key())
                    .col(ColumnDef::new(Database::ConnectionId).uuid().not_null())
                    .col(ColumnDef::new(Database::Name).string().not_null())
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
                            .to(Connection::Table, Connection::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. Create the DatabaseObjects table (Generalized Schema Cache)
        manager
            .create_table(
                Table::create()
                    .table(DatabaseObject::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(DatabaseObject::Id).uuid().primary_key())
                    .col(ColumnDef::new(DatabaseObject::DatabaseId).uuid().not_null())
                    .col(ColumnDef::new(DatabaseObject::Name).string().not_null())
                    .col(
                        ColumnDef::new(DatabaseObject::ObjectType)
                            .string()
                            .not_null(),
                    ) // table, view, function, trigger, etc.
                    .col(ColumnDef::new(DatabaseObject::Definition).text()) // SQL source code
                    .col(ColumnDef::new(DatabaseObject::Metadata).json_binary()) // JSON for columns, params, etc.
                    .col(
                        ColumnDef::new(DatabaseObject::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(DatabaseObject::Table, DatabaseObject::DatabaseId)
                            .to(Database::Table, Database::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx-database-object-name-type")
                    .table(DatabaseObject::Table)
                    .col(DatabaseObject::Name)
                    .col(DatabaseObject::ObjectType)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DatabaseObject::Table).to_owned())
            .await?;

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
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum DatabaseObject {
    Table,
    Id,
    DatabaseId,
    Name,
    ObjectType,
    Definition,
    Metadata,
    CreatedAt,
}

#[derive(Iden)]
enum Connection {
    Table,
    Id,
}
