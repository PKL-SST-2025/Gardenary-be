use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use crate::services::auth_service::AuthService;
use crate::dtos::auth_dto::{RegisterDTO, LoginDTO,};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse<T> {
    status: String,
    message: String,
    data: Option<T>,
}

// Helper function to extract token from Authorization header
fn extract_token(req: &HttpRequest) -> Result<String, String> {
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Ok(auth_str[7..].to_string());
            }
        }
    }
    Err("Missing or invalid authorization header".to_string())
}

// ========== POSTGRES ==========

#[post("/pg/auth/register")]
pub async fn register_pg(
    svc: web::Data<AuthService>,
    body: web::Json<RegisterDTO>
) -> impl Responder {
    match svc.register_pg(body.0).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "User registered successfully".to_string(),
            data: Some(response),
        }),
        Err(err) => HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[post("/pg/auth/login")]
pub async fn login_pg(
    svc: web::Data<AuthService>,
    body: web::Json<LoginDTO>
) -> impl Responder {
    match svc.login_pg(&body.email, &body.password).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Login successful".to_string(),
            data: Some(response),
        }),
        Err(err) => HttpResponse::Unauthorized().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/pg/auth/me")]
pub async fn get_me_pg(
    svc: web::Data<AuthService>,
    req: HttpRequest
) -> impl Responder {
    let token = match extract_token(&req) {
        Ok(token) => token,
        Err(err) => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    };

    let user_id = match svc.verify_token(&token) {
        Ok(id) => id,
        Err(err) => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    };

    match svc.get_user_by_id_pg(user_id).await {
        Ok(user) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "User data retrieved successfully".to_string(),
            data: Some(user),
        }),
        Err(err) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

// ========== SUPABASE ==========

#[post("/sb/auth/register")]
pub async fn register_sb(
    svc: web::Data<AuthService>,
    body: web::Json<RegisterDTO>
) -> impl Responder {
    match svc.register_sb(body.0).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "User registered successfully (Supabase)".to_string(),
            data: Some(response),
        }),
        Err(err) => HttpResponse::BadRequest().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[post("/sb/auth/login")]
pub async fn login_sb(
    svc: web::Data<AuthService>,
    body: web::Json<LoginDTO>
) -> impl Responder {
    match svc.login_sb(&body.email, &body.password).await {
        Ok(response) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "Login successful (Supabase)".to_string(),
            data: Some(response),
        }),
        Err(err) => HttpResponse::Unauthorized().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}

#[get("/sb/auth/me")]
pub async fn get_me_sb(
    svc: web::Data<AuthService>,
    req: HttpRequest
) -> impl Responder {
    let token = match extract_token(&req) {
        Ok(token) => token,
        Err(err) => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    };

    let user_id = match svc.verify_token(&token) {
        Ok(id) => id,
        Err(err) => return HttpResponse::Unauthorized().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    };

    match svc.get_user_by_id_sb(user_id).await {
        Ok(user) => HttpResponse::Ok().json(ApiResponse {
            status: "success".to_string(),
            message: "User data retrieved successfully (Supabase)".to_string(),
            data: Some(user),
        }),
        Err(err) => HttpResponse::NotFound().json(ApiResponse::<()> {
            status: "error".to_string(),
            message: err,
            data: None,
        }),
    }
}