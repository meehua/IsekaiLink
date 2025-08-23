use axum::{
    response::{Redirect, IntoResponse},
    http::{StatusCode, HeaderMap, header},
    middleware::{Next},
    extract::{Request, Extension},
};
use std::{sync::{Arc}};
use axum::extract::State;
use crate::types::app_state::AppState;
use crate::types::session_store::SessionStore;

// 鉴权函数
pub(crate) async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    // 1. 从cookie中获取token
    let token = req.headers()
        .get("Cookie")
        .and_then(|c| c.to_str().ok())
        .and_then(|c| parse_session_token(c))
        .unwrap_or_default()
        .clone();

    // 2. 验证会话（安全持有锁）
    let username = {
        state.sessions.get_user(&token)
    };

    match username {
        Some(username) => {
            // 3. 添加用户名到请求扩展，用于将通过验证的用户名传递给后续执行的 handlers
            req.extensions_mut().insert(username);
            // 4. 释放锁后执行后续请求
            next.run(req).await
        }
        None => {
            // 返回401未授权错误
            StatusCode::UNAUTHORIZED.into_response()
        }
    }
}


async fn logout(Extension(sessions): Extension<SessionStore>) -> impl IntoResponse {
    // 清除会话 (实际应用中应从请求中获取token)

    // 清除cookie并重定向
    let mut headers = HeaderMap::new();
    headers.insert(header::SET_COOKIE,
                   "session_token=; Path=/; Max-Age=0".parse().unwrap());

    (headers, Redirect::to("/"))
}

// 辅助函数：从cookie字符串中解析session_token
fn parse_session_token(cookie_str: &str) -> Option<String> {
    cookie_str.split(';')
        .map(|s| s.trim())
        .find(|s| s.starts_with("session_token="))
        .and_then(|s| s.splitn(2, '=').nth(1))
        .map(|s| s.to_string())
}