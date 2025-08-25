use actix_web::{HttpResponse, web};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::Serialize;
use crate::entity::attendance;
use crate::models::{AttendanceDto, ClockRequest};
use crate::responses::ApiResponse;
use std::collections::HashMap;

#[derive(Serialize)]
struct ErrorResponse<'a> {
    status: &'a str,
    message: &'a str,
}

pub async fn clock_in(
    db: web::Data<sea_orm::DatabaseConnection>,
    payload: web::Json<ClockRequest>,
) -> HttpResponse {
    let user = payload.user_id.trim();
    if user.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid user_id"));
    }

    // Cek apakah user sudah clock in tapi belum clock out
    match attendance::Entity::find()
        .filter(attendance::Column::UserId.eq(user))
        .filter(attendance::Column::ClockOutTime.is_null())
        .one(db.get_ref())
        .await
    {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "User already clocked in and has not clocked out yet",
            ));
        }
        Ok(None) => {}
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(&format!("DB error: {}", e)));
        }
    }

    let now = Utc::now().naive_utc();
    let model = attendance::ActiveModel {
        user_id: Set(user.to_string()),
        clock_in_time: Set(now),
        clock_out_time: Set(None),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };

    match model.insert(db.get_ref()).await {
        Ok(inserted) => {
            let dto = AttendanceDto {
                id: inserted.id,
                user_id: inserted.user_id,
                clock_in_time: inserted.clock_in_time,
                clock_out_time: inserted.clock_out_time,
                created_at: inserted.created_at,
                updated_at: inserted.updated_at,
            };
            HttpResponse::Ok().json(ApiResponse::success("Clock-in recorded", Some(dto)))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("Insert error: {}", e))),
    }
}

pub async fn clock_out(
    db: web::Data<sea_orm::DatabaseConnection>,
    payload: web::Json<ClockRequest>,
) -> HttpResponse {
    let user = payload.user_id.trim();
    if user.is_empty() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error("Invalid user_id"));
    }

    // Cari clock-in terakhir yang belum clock-out
    match attendance::Entity::find()
        .filter(attendance::Column::UserId.eq(user))
        .filter(attendance::Column::ClockOutTime.is_null())
        .order_by_desc(attendance::Column::ClockInTime)
        .one(db.get_ref())
        .await
    {
        Ok(Some(row)) => {
            let now = Utc::now().naive_utc();
            let mut active: attendance::ActiveModel = row.into();
            active.clock_out_time = Set(Some(now));
            active.updated_at = Set(Some(now));

            match active.update(db.get_ref()).await {
                Ok(updated) => {
                    let dto = AttendanceDto {
                        id: updated.id,
                        user_id: updated.user_id,
                        clock_in_time: updated.clock_in_time,
                        clock_out_time: updated.clock_out_time,
                        created_at: updated.created_at,
                        updated_at: updated.updated_at,
                    };
                    HttpResponse::Ok().json(ApiResponse::success("Clock-out recorded", Some(dto)))
                }
                Err(e) => HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error(&format!("Update error: {}", e))),
            }
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "No active clock-in session found for this user",
        )),
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("DB error: {}", e))),
    }
}


pub async fn get_history(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let uid = query.get("user_id").map(|s| s.as_str());
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(50);

    let mut find = attendance::Entity::find().order_by_desc(attendance::Column::ClockInTime);
    if let Some(u) = uid {
        find = find.filter(attendance::Column::UserId.eq(u));
    }

    match find.limit(limit).all(db.get_ref()).await {
        Ok(rows) => {
            if rows.is_empty() {
                // data → 404 Not Found
                return HttpResponse::NotFound().json(ErrorResponse {
                    status: "error",
                    message: "Attendance record not found",
                });
            }

            let data: Vec<AttendanceDto> = rows
                .into_iter()
                .map(|r| AttendanceDto {
                    id: r.id,
                    user_id: r.user_id,
                    clock_in_time: r.clock_in_time,
                    clock_out_time: r.clock_out_time,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                })
                .collect();

            HttpResponse::Ok().json(ApiResponse::success("History fetched", Some(data)))
        }
        Err(e) => HttpResponse::InternalServerError().json(ErrorResponse {
            status: "error",
            message: &format!("DB error: {}", e),
        }),
    }
}