pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_attendance_table;
mod m20220102_000002_seed_attendance;
mod m20220103_000003_create_banner_table;
mod m20220104_000004_seed_banner;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_attendance_table::Migration),
            Box::new(m20220102_000002_seed_attendance::Migration),
            Box::new(m20220103_000003_create_banner_table::Migration),
            Box::new(m20220104_000004_seed_banner::Migration),
        ]
    }
}
