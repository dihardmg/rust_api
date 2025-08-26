use actix_web::{HttpResponse, web};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use std::path::Path;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::Serialize;
use crate::entity::{attendance, banner};
use crate::models::{AttendanceDto, ClockRequest, CreateBannerRequest, UpdateBannerRequest, BannerDto};
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

// Banner handlers
pub async fn upload_banner_image(mut multipart: Multipart) -> HttpResponse {
    while let Some(field) = multipart.try_next().await.unwrap_or(None) {
        let content_disposition = field.content_disposition();
        
        if let Some(content_disposition) = content_disposition {
            if let Some(filename) = content_disposition.get_filename() {
            let file_extension = Path::new(filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("jpg");
            
            // Generate unique filename
            let new_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
            let filepath = format!("uploads/banners/{}", new_filename);
            
            // Create uploads directory if it doesn't exist
            if let Err(_) = tokio::fs::create_dir_all("uploads/banners").await {
                return HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to create upload directory"));
            }
            
            // Save file
            match File::create(&filepath).await {
                Ok(mut f) => {
                    let mut field_stream = field;
                    while let Some(chunk) = field_stream.try_next().await.unwrap_or(None) {
                        if f.write_all(&chunk).await.is_err() {
                            return HttpResponse::InternalServerError()
                                .json(ApiResponse::<()>::error("Failed to write file"));
                        }
                    }
                    
                    let response_data = serde_json::json!({
                        "image_url": format!("/{}", filepath)
                    });
                    
                    return HttpResponse::Ok()
                        .json(ApiResponse::success("Image uploaded successfully", Some(response_data)));
                }
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse::<()>::error("Failed to create file"));
                }
            }
            }
        }
    }
    
    HttpResponse::BadRequest().json(ApiResponse::<()>::error("No file provided"))
}

pub async fn create_banner(
    db: web::Data<sea_orm::DatabaseConnection>,
    payload: web::Json<CreateBannerRequest>,
) -> HttpResponse {
    let start_date = match chrono::NaiveDateTime::parse_from_str(&payload.start_date, "%Y-%m-%d %H:%M:%S") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("Invalid start_date format. Use: YYYY-MM-DD HH:MM:SS"));
        }
    };
    
    let end_date = match chrono::NaiveDateTime::parse_from_str(&payload.end_date, "%Y-%m-%d %H:%M:%S") {
        Ok(date) => date,
        Err(_) => {
            return HttpResponse::BadRequest()
                .json(ApiResponse::<()>::error("Invalid end_date format. Use: YYYY-MM-DD HH:MM:SS"));
        }
    };
    
    if start_date >= end_date {
        return HttpResponse::BadRequest()
            .json(ApiResponse::<()>::error("start_date must be before end_date"));
    }
    
    let now = Utc::now().naive_utc();
    let model = banner::ActiveModel {
        title: Set(payload.title.clone()),
        content: Set(payload.content.clone()),
        image_url: Set(None),
        start_date: Set(start_date),
        end_date: Set(end_date),
        is_active: Set(true),
        created_at: Set(Some(now)),
        updated_at: Set(Some(now)),
        ..Default::default()
    };
    
    match model.insert(db.get_ref()).await {
        Ok(inserted) => {
            let dto = BannerDto {
                id: inserted.id,
                title: inserted.title,
                content: inserted.content,
                image_url: inserted.image_url,
                start_date: inserted.start_date,
                end_date: inserted.end_date,
                is_active: inserted.is_active,
                created_at: inserted.created_at,
                updated_at: inserted.updated_at,
            };
            HttpResponse::Created().json(ApiResponse::success("Banner created", Some(dto)))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("Insert error: {}", e))),
    }
}

pub async fn update_banner_image(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
    mut multipart: Multipart,
) -> HttpResponse {
    let banner_id = path.into_inner();
    
    // Check if banner exists
    let banner_result = banner::Entity::find_by_id(banner_id).one(db.get_ref()).await;
    let banner = match banner_result {
        Ok(Some(b)) => b,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("Banner not found"));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(&format!("DB error: {}", e)));
        }
    };
    
    while let Some(field) = multipart.try_next().await.unwrap_or(None) {
        let content_disposition = field.content_disposition();
        
        if let Some(content_disposition) = content_disposition {
            if let Some(filename) = content_disposition.get_filename() {
                let file_extension = Path::new(filename)
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("jpg");
            
            let new_filename = format!("{}.{}", Uuid::new_v4(), file_extension);
            let filepath = format!("uploads/banners/{}", new_filename);
            
            if let Err(_) = tokio::fs::create_dir_all("uploads/banners").await {
                return HttpResponse::InternalServerError()
                    .json(ApiResponse::<()>::error("Failed to create upload directory"));
            }
            
            match File::create(&filepath).await {
                Ok(mut f) => {
                    let mut field_stream = field;
                    while let Some(chunk) = field_stream.try_next().await.unwrap_or(None) {
                        if f.write_all(&chunk).await.is_err() {
                            return HttpResponse::InternalServerError()
                                .json(ApiResponse::<()>::error("Failed to write file"));
                        }
                    }
                    
                    // Update banner with image URL
                    let mut active: banner::ActiveModel = banner.into();
                    active.image_url = Set(Some(format!("/{}", filepath)));
                    active.updated_at = Set(Some(Utc::now().naive_utc()));
                    
                    match active.update(db.get_ref()).await {
                        Ok(updated) => {
                            let dto = BannerDto {
                                id: updated.id,
                                title: updated.title,
                                content: updated.content,
                                image_url: updated.image_url,
                                start_date: updated.start_date,
                                end_date: updated.end_date,
                                is_active: updated.is_active,
                                created_at: updated.created_at,
                                updated_at: updated.updated_at,
                            };
                            return HttpResponse::Ok()
                                .json(ApiResponse::success("Banner image updated", Some(dto)));
                        }
                        Err(e) => {
                            return HttpResponse::InternalServerError()
                                .json(ApiResponse::<()>::error(&format!("Update error: {}", e)));
                        }
                    }
                }
                Err(_) => {
                    return HttpResponse::InternalServerError()
                        .json(ApiResponse::<()>::error("Failed to create file"));
                }
            }
            }
        }
    }
    
    HttpResponse::BadRequest().json(ApiResponse::<()>::error("No file provided"))
}

pub async fn get_banners(
    db: web::Data<sea_orm::DatabaseConnection>,
    query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let limit = query
        .get("limit")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(50);
    
    match banner::Entity::find()
        .order_by_desc(banner::Column::CreatedAt)
        .limit(limit)
        .all(db.get_ref())
        .await
    {
        Ok(rows) => {
            let data: Vec<BannerDto> = rows
                .into_iter()
                .map(|r| BannerDto {
                    id: r.id,
                    title: r.title,
                    content: r.content,
                    image_url: r.image_url,
                    start_date: r.start_date,
                    end_date: r.end_date,
                    is_active: r.is_active,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                })
                .collect();
            
            HttpResponse::Ok().json(ApiResponse::success("Banners fetched", Some(data)))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("DB error: {}", e))),
    }
}

pub async fn get_active_banner(
    db: web::Data<sea_orm::DatabaseConnection>,
) -> HttpResponse {
    let now = Utc::now().naive_utc();
    
    match banner::Entity::find()
        .filter(banner::Column::IsActive.eq(true))
        .filter(banner::Column::StartDate.lte(now))
        .filter(banner::Column::EndDate.gt(now)) // EndDate harus lebih besar dari sekarang (belum expired)
        .order_by_asc(banner::Column::EndDate) // Prioritas banner yang akan expired lebih dulu
        .order_by_desc(banner::Column::CreatedAt) // Jika end_date sama, pilih yang terbaru dibuat
        .one(db.get_ref())
        .await
    {
        Ok(Some(banner)) => {
            let dto = BannerDto {
                id: banner.id,
                title: banner.title,
                content: banner.content,
                image_url: banner.image_url,
                start_date: banner.start_date,
                end_date: banner.end_date,
                is_active: banner.is_active,
                created_at: banner.created_at,
                updated_at: banner.updated_at,
            };
            HttpResponse::Ok().json(ApiResponse::success("Active banner found", Some(dto)))
        }
        Ok(None) => {
            // Return default banner
            let default_banner = BannerDto {
                id: 0,
                title: Some("Welcome".to_string()),
                content: "This is the default banner announcement.".to_string(),
                image_url: None,
                start_date: now,
                end_date: now,
                is_active: true,
                created_at: Some(now),
                updated_at: Some(now),
            };
            HttpResponse::Ok().json(ApiResponse::success("Default banner", Some(default_banner)))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("DB error: {}", e))),
    }
}

pub async fn update_banner(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
    payload: web::Json<UpdateBannerRequest>,
) -> HttpResponse {
    let banner_id = path.into_inner();
    
    let banner_result = banner::Entity::find_by_id(banner_id).one(db.get_ref()).await;
    let banner = match banner_result {
        Ok(Some(b)) => b,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(ApiResponse::<()>::error("Banner not found"));
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ApiResponse::<()>::error(&format!("DB error: {}", e)));
        }
    };
    
    let mut active: banner::ActiveModel = banner.into();
    
    if let Some(ref title) = payload.title {
        active.title = Set(Some(title.clone()));
    }
    
    if let Some(ref content) = payload.content {
        active.content = Set(content.clone());
    }
    
    if let Some(ref start_date_str) = payload.start_date {
        match chrono::NaiveDateTime::parse_from_str(start_date_str, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => active.start_date = Set(date),
            Err(_) => {
                return HttpResponse::BadRequest()
                    .json(ApiResponse::<()>::error("Invalid start_date format. Use: YYYY-MM-DD HH:MM:SS"));
            }
        }
    }
    
    if let Some(ref end_date_str) = payload.end_date {
        match chrono::NaiveDateTime::parse_from_str(end_date_str, "%Y-%m-%d %H:%M:%S") {
            Ok(date) => active.end_date = Set(date),
            Err(_) => {
                return HttpResponse::BadRequest()
                    .json(ApiResponse::<()>::error("Invalid end_date format. Use: YYYY-MM-DD HH:MM:SS"));
            }
        }
    }
    
    if let Some(is_active) = payload.is_active {
        active.is_active = Set(is_active);
    }
    
    active.updated_at = Set(Some(Utc::now().naive_utc()));
    
    match active.update(db.get_ref()).await {
        Ok(updated) => {
            let dto = BannerDto {
                id: updated.id,
                title: updated.title,
                content: updated.content,
                image_url: updated.image_url,
                start_date: updated.start_date,
                end_date: updated.end_date,
                is_active: updated.is_active,
                created_at: updated.created_at,
                updated_at: updated.updated_at,
            };
            HttpResponse::Ok().json(ApiResponse::success("Banner updated", Some(dto)))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("Update error: {}", e))),
    }
}

pub async fn delete_banner(
    db: web::Data<sea_orm::DatabaseConnection>,
    path: web::Path<i32>,
) -> HttpResponse {
    let banner_id = path.into_inner();
    
    match banner::Entity::delete_by_id(banner_id).exec(db.get_ref()).await {
        Ok(res) => {
            if res.rows_affected > 0 {
                HttpResponse::Ok().json(ApiResponse::<()>::success("Banner deleted", None))
            } else {
                HttpResponse::NotFound().json(ApiResponse::<()>::error("Banner not found"))
            }
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(ApiResponse::<()>::error(&format!("Delete error: {}", e))),
    }
}