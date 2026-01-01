pub use sea_orm_migration::prelude::*;

mod m20251231_013331_create_tasks_and_tags_tables;
mod m20260101_010000_add_completed_at_to_tasks;
pub mod seeder;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251231_013331_create_tasks_and_tags_tables::Migration),
            Box::new(m20260101_010000_add_completed_at_to_tasks::Migration),
        ]
    }
}
