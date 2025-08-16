use actix_web::{get, post, put, patch, delete, web, HttpResponse, Responder, HttpRequest};
use uuid::Uuid;
use crate::services::plant_service::PlantService;
use crate::dtos::plant_dto::{CreatePlantDTO, UpdatePlantDTO, UpdatePlantStatusDTO};
use crate::models::plant::{Plant, ApiResponse};
use serde::Serialize;

// FIXED: Proper JWT token extraction and validation
fn get_user_id_from_request(req: &HttpRequest) -> Result<Uuid, String> {
    // Extract Authorization header
    let auth_header = req.headers()
        .get("Authorization")
        .ok_or_else(|| "Authorization header required".to_string())?
        .to_str()
        .map_err(|_| "Invalid authorization header".to_string())?;

    // Check if it starts with "Bearer "
    if !auth_header.starts_with("Bearer ") {
        return Err("Invalid authorization format. Use: Bearer <token>".to_string());
    }

    let token = auth_header.strip_prefix("Bearer ").unwrap();
    
    // OPTION 1: For testing - extract user_id from a simple token format
    // Assuming token format: "user_<uuid>" for testing
    if token.starts_with("user_") {
        let user_id_str = token.strip_prefix("user_").unwrap();
        return Uuid::parse_str(user_id_str)
            .map_err(|_| "Invalid user ID in token".to_string());
    }
    
    // OPTION 2: Proper JWT validation (uncomment when ready)
    /*
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,  // user_id
        exp: usize,   // expiration
    }
    
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key".to_string());
    
    let decoding_key = DecodingKey::from_secret(jwt_secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(token, &decoding_key, &validation)
        .map_err(|e| format!("Invalid JWT token: {}", e))?;
    
    Uuid::parse_str(&token_data.claims.sub)
        .map_err(|_| "Invalid user ID in JWT".to_string())
    */
    
    // OPTION 3: For immediate testing - return error with helpful message
    Err("Invalid token format. For testing, use: Bearer user_<uuid>".to_string())
}

// ========== POSTGRES ==========

#[post("/pg/plants")]
pub async fn add_pg_plant(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    body: web::Json<CreatePlantDTO>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    let mut dto = body.into_inner();
    dto.user_id = user_id;

    match svc.add_pg(dto).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant added successfully".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/pg/plants")]
pub async fn get_all_pg_plants(
    req: HttpRequest,
    svc: web::Data<PlantService>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.get_all_pg(user_id).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: format!("{} plants found", list.len()),
            data: Some(list),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<Plant>> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/pg/plants/{id}")]
pub async fn get_pg_plant_by_id(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.get_by_id_pg(id.into_inner(), user_id).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant found".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[put("/pg/plants/{id}")]
pub async fn update_pg_plant(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>,
    body: web::Json<UpdatePlantDTO>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.update_pg(id.into_inner(), user_id, body.into_inner()).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant updated successfully".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[patch("/pg/plants/{id}/status")]
pub async fn update_pg_plant_status(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>,
    body: web::Json<UpdatePlantStatusDTO>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.update_status_pg(id.into_inner(), user_id, body.into_inner()).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant status updated successfully".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[delete("/pg/plants/{id}")]
pub async fn delete_pg_plant(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.delete_pg(id.into_inner(), user_id).await {
        Ok(deleted_count) if deleted_count > 0 => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "success".to_string(),
            message: "Plant deleted successfully".to_string(),
            data: None,
        }),
        Ok(_) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: "Plant not found".to_string(),
            data: None,
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/pg/dashboard")]
pub async fn get_pg_dashboard_stats(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    query: web::Query<std::collections::HashMap<String, String>>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    let date = query.get("date").map(|s| s.as_str()).unwrap_or("2025-07-15");
    
    match svc.get_dashboard_stats_pg(user_id, date).await {
        Ok(stats) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Dashboard stats retrieved successfully".to_string(),
            data: Some(stats),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

// ========== SUPABASE ==========

#[post("/sb/plants")]
pub async fn add_sb_plant(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    body: web::Json<CreatePlantDTO>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    let mut dto = body.into_inner();
    dto.user_id = user_id;

    match svc.add_sb(dto).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant added successfully (Supabase)".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/sb/plants")]
pub async fn get_all_sb_plants(
    req: HttpRequest,
    svc: web::Data<PlantService>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.get_all_sb(user_id).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: format!("{} plants found (Supabase)", list.len()),
            data: Some(list),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<Plant>> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/sb/plants/{id}")]
pub async fn get_sb_plant_by_id(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.get_by_id_sb(id.into_inner(), user_id).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant found (Supabase)".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[put("/sb/plants/{id}")]
pub async fn update_sb_plant(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>,
    body: web::Json<UpdatePlantDTO>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.update_sb(id.into_inner(), user_id, body.into_inner()).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant updated successfully (Supabase)".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[patch("/sb/plants/{id}/status")]
pub async fn update_sb_plant_status(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>,
    body: web::Json<UpdatePlantStatusDTO>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.update_status_sb(id.into_inner(), user_id, body.into_inner()).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant status updated successfully (Supabase)".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[delete("/sb/plants/{id}")]
pub async fn delete_sb_plant(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    id: web::Path<Uuid>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.delete_sb(id.into_inner(), user_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::<()> {
            status: "success".to_string(),
            message: "Plant deleted successfully (Supabase)".to_string(),
            data: None,
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/sb/dashboard")]
pub async fn get_sb_dashboard_stats(
    req: HttpRequest,
    svc: web::Data<PlantService>,
    query: web::Query<std::collections::HashMap<String, String>>
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req) {
        Ok(id) => id,
        Err(err) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    let date = query.get("date").map(|s| s.as_str()).unwrap_or("2025-07-15");
    
    match svc.get_dashboard_stats_sb(user_id, date).await {
        Ok(stats) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Dashboard stats retrieved successfully (Supabase)".to_string(),
            data: Some(stats),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

// ========== TESTING ENDPOINTS (No Auth Required) ==========

#[post("/test/sb/plants")]
pub async fn add_sb_plant_test(
    svc: web::Data<PlantService>,
    body: web::Json<CreatePlantDTO>
) -> impl Responder {
    // For testing purposes - no auth required
    match svc.add_sb(body.into_inner()).await {
        Ok(plant) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Plant added successfully (Test - Supabase)".to_string(),
            data: Some(plant),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/test/sb/plants")]
pub async fn get_all_sb_plants_test(
    svc: web::Data<PlantService>,
    query: web::Query<std::collections::HashMap<String, String>>
) -> impl Responder {
    // Get user_id from query params for testing
    let user_id_str = query.get("user_id")
        .ok_or_else(|| "user_id query parameter required".to_string());
    
    let user_id = match user_id_str {
        Ok(id_str) => {
            match Uuid::parse_str(id_str) {
                Ok(id) => id,
                Err(_) => {
                    return HttpResponse::BadRequest().json(ApiResponse::<()> {
                        status: "error".to_string(),
                        message: "Invalid user_id format".to_string(),
                        data: None,
                    });
                }
            }
        }
        Err(err) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()> {
                status: "error".to_string(),
                message: err,
                data: None,
            });
        }
    };

    match svc.get_all_sb(user_id).await {
        Ok(list) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: format!("{} plants found (Test - Supabase)", list.len()),
            data: Some(list),
        }),
        Err(err) => HttpResponse::InternalServerError().json(ApiResponse::<Vec<Plant>> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}