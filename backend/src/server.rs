use crate::db::DbClient;
use crate::handlers::login;
use crate::handlers::login::login;
use crate::middlewares::auth;
use crate::types::app_state::AppState;
use crate::types::session_store::SessionStore;
use axum::{
    Router,
    extract::{Extension, Form, Json, Path, Query, State},
    http::status::StatusCode,
    middleware,
    routing::{get, post},
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn run() {
    // 初始化数据结构
    let db_client = DbClient::connect().await.unwrap();
    let sessions = SessionStore::new();

    // 创建应用状态
    let app_state = Arc::new(AppState {
        db_client,
        sessions,
    });

    // 建立路由

    // 实时鉴权的路由
    let auth_routes = Router::new()
        .route("/api", get(|| async { "Hello, World!" }))
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth::auth_middleware,
        ))
        .with_state(app_state.clone());

    // 不具备实时鉴权
    let other_routes = Router::new()
        .route("/api/auth/login", post(login::login))
        .with_state(app_state.clone());

    // 合并路由
    let app = Router::new()
        .merge(auth_routes)
        .merge(other_routes)
        .fallback(|| async { StatusCode::NOT_FOUND });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:30022")
        .await
        .unwrap();
    println!(
        "server started on http://{}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

pub fn main() {
    run()
}

#[cfg(test)]
mod tests {}
