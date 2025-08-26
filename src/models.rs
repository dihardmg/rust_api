use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct ClockRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct AttendanceDto {
    pub id: i32,
    pub user_id: String,
    pub clock_in_time: NaiveDateTime,
    pub clock_out_time: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct CreateBannerRequest {
    pub title: Option<String>,
    pub content: String,
    pub start_date: String, // Format: "2024-01-01 10:00:00"
    pub end_date: String,   // Format: "2024-12-31 23:59:59"
}

#[derive(Deserialize)]
pub struct UpdateBannerRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize)]
pub struct BannerDto {
    pub id: i32,
    pub title: Option<String>,
    pub content: String,
    pub image_url: Option<String>,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub is_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
