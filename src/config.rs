use std::env;
use deadpool_postgres::{Config, Pool};

pub fn get_pg_pool() -> Pool {
    let mut cfg = Config::new();
    cfg.host = Some(env::var("PG_HOST").expect("PG_HOST not set"));
    cfg.user = Some(env::var("PG_USER").expect("PG_USER not set"));
    cfg.password = env::var("PG_PASS").ok();
    cfg.dbname = Some(env::var("PG_DB").expect("PG_DB not set"));
    cfg.create_pool(None, tokio_postgres::NoTls).unwrap()
}

pub fn get_supabase_url() -> String {
    env::var("SUPABASE_URL").expect("SUPABASE_URL not set")
}