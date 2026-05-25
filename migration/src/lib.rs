use chrono::{NaiveDateTime, Timelike, Utc};
pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20250925_000001_create_connection_table;
mod m20250925_000002_create_tab_table;
mod m20250925_000003_create_saved_query_table;
mod m20250925_000004_create_query_history_table;
mod m20250925_000005_create_preference_table;
mod m20250925_000006_create_workspace_table;
mod m20250925_000007_create_database_table;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250925_000006_create_workspace_table::Migration),
            Box::new(m20250925_000001_create_connection_table::Migration),
            Box::new(m20250925_000002_create_tab_table::Migration),
            Box::new(m20250925_000003_create_saved_query_table::Migration),
            Box::new(m20250925_000004_create_query_history_table::Migration),
            Box::new(m20250925_000005_create_preference_table::Migration),
            Box::new(m20250925_000007_create_database_table::Migration),
        ]
    }
}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_local().with_nanosecond(0).unwrap()
}
