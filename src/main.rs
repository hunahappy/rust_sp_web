use axum::{
    Router,
    routing::{get, post},
};
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tower_sessions::cookie::time::Duration;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
use tracing::info;

mod comm;
mod routes;

#[derive(Clone)]
struct AppState {
    db: sqlx::PgPool,
}

#[tokio::main]
async fn main() {
    let file = File::open("rust_sp_web.conf").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let json: Value = serde_json::from_str(&contents).unwrap(); // 문자열 → Value

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////

    tracing_subscriber::fmt().with_env_filter("info").init();
    info!("SERVER STARTED");

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&format!(
            "postgres://{}:{}@{}/{}",
            json["db_id"].as_str().unwrap(),
            json["db_password"].as_str().unwrap(),
            json["db_server"].as_str().unwrap(),
            json["db_db"].as_str().unwrap()
        ))
        .await
        .expect("db connect fail");

    let state = AppState { db: db.clone() };

    // 세션 저장소 (PostgreSQL)
    let store = PostgresStore::new(db.clone());
    store.migrate().await.unwrap(); // 테이블 생성

    let session_layer = SessionManagerLayer::new(store)
        .with_secure(true)
        .with_expiry(Expiry::OnInactivity(Duration::hours(3)));

    /////////////////////////////////////////////////////////////////////////////////////////////////////////////

    let app = Router::new()
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////
        .route("/", get(routes::login::get_login))
        .route("/login/{id}", post(routes::login::post_login))
        .route("/report_th_1", get(routes::report_th_1::get_report_th_1))
        .route("/report_th_1/{id}", post(routes::report_th_1::post_report_th_1))
        .route("/system_info", get(routes::system_info::get_system_info))
        .route("/system_info/{id}", post(routes::system_info::post_system_info))
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////
        .layer(
            TraceLayer::new_for_http().on_request(|req: &axum::http::Request<_>, _span: &tracing::Span| {
                let ip = req
                    .headers()
                    .get("x-forwarded-for")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("-");

                tracing::info!("--> {} {} from {}", req.method(), req.uri().path(), ip);
            }),
        )
        .layer(session_layer)
        // db pool을 상태로 추가
        .with_state(state)
        // /static/* 경로로 CSS, JS, 이미지 제공
        .nest_service("/public", ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:10201").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
