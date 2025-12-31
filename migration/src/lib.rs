pub use sea_orm_migration::prelude::*;

mod m20251231_013331_create_tasks_and_tags_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(
            m20251231_013331_create_tasks_and_tags_tables::Migration,
        )]
    }
}
