pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260617_161716_create_user_table;
mod m20260718_000001_create_threads_and_replies;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            //Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260617_161716_create_user_table::Migration),
            Box::new(m20260718_000001_create_threads_and_replies::Migration),
        ]
    }
}
