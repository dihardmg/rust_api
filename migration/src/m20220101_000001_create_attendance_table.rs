use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Create attendance table
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Attendance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attendance::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Attendance::UserId).string_len(100).not_null())
                    .col(ColumnDef::new(Attendance::ClockInTime).date_time().not_null())
                    .col(ColumnDef::new(Attendance::ClockOutTime).date_time().null())
                    .col(ColumnDef::new(Attendance::CreatedAt).date_time().null())
                    .col(ColumnDef::new(Attendance::UpdatedAt).date_time().null())
                    .to_owned(),
            )
            .await
    }

    // Drop attendance table
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attendance::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Attendance {
    Table,
    Id,
    UserId,
    ClockInTime,
    ClockOutTime,
    CreatedAt,
    UpdatedAt,
}
