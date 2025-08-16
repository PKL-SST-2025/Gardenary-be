mod config;
mod dtos;
mod models;
mod repositories;
mod services;
mod handlers;

use actix_web::{App, HttpServer, web, middleware::Logger};
use actix_cors::Cors;
use services::plant_service::PlantService;
use services::auth_service::AuthService;
use repositories::plant_postgres::PlantPostgresRepo;
use repositories::plant_supabase::PlantSupabaseRepo;
use repositories::auth_postgres::AuthPostgresRepo;
use repositories::auth_supabase::AuthSupabaseRepo;
use handlers::plant_handler::*;
use handlers::auth_handler::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    // Validasi env variable
    let supabase_url = std::env::var("SUPABASE_URL")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "SUPABASE_URL not set"))?;
    let supabase_key = std::env::var("SUPABASE_KEY")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "SUPABASE_KEY not set"))?;
    let jwt_secret = std::env::var("JWT_SECRET")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "JWT_SECRET not set"))?;

    // Log config untuk debug
    println!("Supabase URL: {}", supabase_url);
    println!("Supabase KEY: {}", &supabase_key[..6]); // hanya 6 karakter pertama
    println!("JWT Secret: {}", &jwt_secret[..6]); // hanya 6 karakter pertama

    let pg_pool = config::get_pg_pool();
    
    // Plant services
    let plant_pg_repo = PlantPostgresRepo { pool: pg_pool.clone() };
    let plant_sb_repo = PlantSupabaseRepo {
        project_url: supabase_url.clone(),
        api_key: supabase_key.clone(),
    };
    let plant_svc = web::Data::new(PlantService { 
        pg_repo: plant_pg_repo, 
        sb_repo: plant_sb_repo 
    });

    // Auth services
    let auth_pg_repo = AuthPostgresRepo { pool: pg_pool };
    let auth_sb_repo = AuthSupabaseRepo {
        project_url: supabase_url,
        api_key: supabase_key,
    };
    let auth_svc = web::Data::new(AuthService {
        pg_repo: auth_pg_repo,
        sb_repo: auth_sb_repo,
        jwt_secret,
    });

    println!("🚀 Plant Management Server starting on http://127.0.0.1:8081");

    HttpServer::new(move || {
        // Konfigurasi CORS
        let cors = Cors::default()
            .allowed_origin("http://localhost:3001")  // Frontend URL
            .allowed_origin("http://localhost:3000")  // Frontend URL
            .allowed_origin("http://127.0.0.1:3000")  // Alternative localhost
            .allowed_origin("http://localhost:5173")  // Vite default port
            .allowed_origin("http://127.0.0.1:5173")  // Alternative Vite port
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                "accept",
                "authorization", 
                "content-type",
                "user-agent",
                "x-requested-with"
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)  // Tambahkan CORS middleware
            .wrap(Logger::default())  // Logger untuk debugging
            .app_data(plant_svc.clone())
            .app_data(auth_svc.clone())
            // Plant endpoints - Postgres
            .service(add_pg_plant)
            .service(get_all_pg_plants)
            .service(get_pg_plant_by_id)
            .service(update_pg_plant)
            .service(update_pg_plant_status)
            .service(delete_pg_plant)
            .service(get_pg_dashboard_stats)
            // Plant endpoints - Supabase
            .service(add_sb_plant)
            .service(get_all_sb_plants)
            .service(get_sb_plant_by_id)
            .service(update_sb_plant)
            .service(update_sb_plant_status)
            .service(delete_sb_plant)
            .service(get_sb_dashboard_stats)
            // Auth endpoints - Postgres
            .service(register_pg)
            .service(login_pg)
            .service(get_me_pg)
            // Auth endpoints - Supabase
            .service(register_sb)
            .service(login_sb)
            .service(get_me_sb)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
