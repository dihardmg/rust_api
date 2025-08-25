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
