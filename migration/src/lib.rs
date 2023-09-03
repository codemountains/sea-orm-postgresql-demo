pub use sea_orm_migration::prelude::*;

mod m20230903_084439_create_table_users;
mod m20230903_084447_create_table_todos;

pub struct Migrator;

// DATABASE_URL="postgresql://postgres:postgres@localhost:5432/seaorm_db" sea-orm-cli migrate refresh

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230903_084439_create_table_users::Migration),
            Box::new(m20230903_084447_create_table_todos::Migration),
        ]
    }
}
