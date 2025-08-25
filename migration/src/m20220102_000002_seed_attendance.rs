use sea_orm_migration::prelude::*;
use sea_orm::Statement;
use chrono::{Utc, Duration};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let now = Utc::now().naive_utc();
        let mut inserts = vec![];

        for i in 1..=100 {
            let user_id = format!("user{:03}", i % 10 + 1); // 10 user dipakai berulang
            let clock_in = now - chrono::Duration::days(i as i64);
            let clock_out = clock_in + chrono::Duration::hours(8);

            inserts.push(format!(
                "('{}', '{}', '{}', '{}', '{}')",
                user_id,
                clock_in,
                clock_out,
                clock_in,
                clock_out
            ));
        }

        let sql = format!(
            "INSERT INTO attendance (user_id, clock_in_time, clock_out_time, created_at, updated_at) VALUES {};",
            inserts.join(",")
        );

        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                sql,
            ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // hapus semua dummy data (opsional, bisa filter by user_id prefix juga)
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                "DELETE FROM attendance;",
            ))
            .await?;
        Ok(())
    }
}
