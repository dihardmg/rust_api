use sea_orm::Statement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Insert sample banner data using simple approach
        let queries = vec![
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES ('Holiday Sale #1', '🎉 Big Holiday Sale! Up to 70% off everything!', '/uploads/banners/holiday-1.jpg', DATE_SUB(NOW(), INTERVAL 14 DAY), DATE_ADD(NOW(), INTERVAL 7 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES (NULL, '🚀 New Product Launch - Revolutionary Technology!', NULL, DATE_SUB(NOW(), INTERVAL 13 DAY), DATE_ADD(NOW(), INTERVAL 8 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES ('Flash Sale Alert', '⚡ 24-Hour Flash Sale! Don''t miss out!', NULL, DATE_SUB(NOW(), INTERVAL 12 DAY), DATE_ADD(NOW(), INTERVAL 9 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES (NULL, '🔧 Maintenance Notice: Sunday 2-4 AM', '/uploads/banners/maintenance.jpg', DATE_SUB(NOW(), INTERVAL 11 DAY), DATE_ADD(NOW(), INTERVAL 10 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES ('Welcome Banner', '👋 Welcome to our amazing platform!', NULL, DATE_SUB(NOW(), INTERVAL 10 DAY), DATE_ADD(NOW(), INTERVAL 11 DAY), 0, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES (NULL, '🔒 Security Update Available - Please Update!', NULL, DATE_SUB(NOW(), INTERVAL 9 DAY), DATE_ADD(NOW(), INTERVAL 12 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES ('Active Banner Now', '⭐ This banner is currently active and visible!', '/uploads/banners/active.jpg', DATE_SUB(NOW(), INTERVAL 4 DAY), DATE_ADD(NOW(), INTERVAL 30 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES (NULL, '🔮 Future Banner - Will be active soon!', '/uploads/banners/future.jpg', DATE_ADD(NOW(), INTERVAL 1 DAY), DATE_ADD(NOW(), INTERVAL 30 DAY), 1, NOW(), NOW())",
            "INSERT INTO banner (title, content, image_url, start_date, end_date, is_active, created_at, updated_at) VALUES ('Inactive Banner', '❌ This banner is set to inactive', NULL, DATE_SUB(NOW(), INTERVAL 2 DAY), DATE_ADD(NOW(), INTERVAL 18 DAY), 0, NOW(), NOW())",
        ];

        for query in queries {
            let stmt = Statement::from_string(manager.get_database_backend(), query.to_string());
            manager.get_connection().execute(stmt).await.map_err(|e| DbErr::Migration(format!("Failed to execute query: {}", e)))?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Statement::from_string(
            manager.get_database_backend(),
            "DELETE FROM banner".to_string()
        );
        manager.get_connection().execute(stmt).await.map_err(|e| DbErr::Migration(format!("Failed to delete banners: {}", e)))?;
        Ok(())
    }
}