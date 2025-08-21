use axum::{extract::{Form, State}, Extension, Json};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, Mutex}};
use std::ffi::CString;
use axum::http::{header, HeaderMap};
use axum::response::{IntoResponse, Redirect};
use log::error;
use crate::types::session_store::SessionStore;
use crate::types::app_state::AppState;
use crate::db::{DbClient, User};

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    code: i32,
    token: String,
    message: String,
}
#[axum::debug_handler]
pub async fn login(State(state): State<Arc<AppState>>, Form(form): Form<LoginForm>)
    -> Json<LoginResponse> {
    // format!(
    //     "Received form: username={}, password={}",
    //     form.username, form.password
    // )
    // 验证用户名密码
    let username = form.username.clone();
    let password = form.password.clone();
    if verify_pwd_hash(&username, &password, state.db_client.clone()).await {

        let token = state.sessions.add_session(&username);
        let resp = LoginResponse {
            code: 0,
            token,
            message: "ログインできました".to_string(),
        };
        Json(resp)


    } else {
        let token = state.sessions.add_session(&username);
        let resp = LoginResponse {
            code: -1,
            token: "".to_string(),
            message: "ザ~コ♡ ザ~コ♡ ログインに失敗したのだ〜".to_string(),
        };
        Json(resp)
    }
}


async fn verify_pwd_hash (user_name:&str, pw_hash:&str, db_client: DbClient) -> bool{
    let user_info = db_client.get_user_by_username(user_name).await;
    // println!("pwd: {:?}，dbpwd: {:?}",pw_hash, user_info);
    match user_info {
        Ok(user_info) => {user_info.pwd_hash == pw_hash},
        Err(e) => {false}
    }
}
