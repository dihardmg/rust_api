use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Banner::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Banner::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Banner::Title).string_len(255).null())
                    .col(ColumnDef::new(Banner::Content).text().not_null())
                    .col(ColumnDef::new(Banner::ImageUrl).string_len(500).null())
                    .col(ColumnDef::new(Banner::StartDate).date_time().not_null())
                    .col(ColumnDef::new(Banner::EndDate).date_time().not_null())
                    .col(ColumnDef::new(Banner::IsActive).boolean().not_null().default(true))
                    .col(ColumnDef::new(Banner::CreatedAt).date_time().null())
                    .col(ColumnDef::new(Banner::UpdatedAt).date_time().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Banner::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Banner {
    Table,
    Id,
    Title,
    Content,
    ImageUrl,
    StartDate,
    EndDate,
    IsActive,
    CreatedAt,
    UpdatedAt,
}