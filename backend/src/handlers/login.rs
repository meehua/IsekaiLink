use axum::{extract::{Form, State}};
use serde::{Deserialize, Serialize};
use std::{ sync::{Arc}};
use crate::types::app_state::AppState;
use crate::db::{DbClient};
use crate::types::api_response::*;

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String
}
#[axum::debug_handler]
pub async fn login(State(state): State<Arc<AppState>>, Form(form): Form<LoginForm>)
    -> ApiResponse<LoginResponse> {
    // 验证用户名密码
    let username = form.username.clone();
    let password = form.password.clone();
    if verify_pwd_hash(&username, &password, state.db_client.clone()).await {

        let token = state.sessions.add_session(&username);
        let resp = LoginResponse {
            token
        };
        ApiResponse::success(resp)

    } else {
        // 错误响应
        ApiResponse::error(BizCode::Unauthorized, Some("登录失败"))
    }
}


async fn verify_pwd_hash (user_name:&str, pw_hash:&str, db_client: DbClient) -> bool{
    let user_info = db_client.get_user_by_username(user_name).await;
    match user_info {
        Ok(user_info) => {user_info.pwd_hash == pw_hash},
        Err(e) => {false}
    }
}


