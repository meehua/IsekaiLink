use axum::{
    Json,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

// 业务状态码与消息的绑定
#[derive(Debug, Clone, Copy)]
pub enum BizCode {
    Success,      // 成功
    BadRequest,   // 请求错误
    Unauthorized, // 未授权
    Forbidden,    // 禁止访问
    NotFound,     // 资源不存在
    ServerError,  // 服务器错误
}

impl BizCode {
    // 获取业务码值
    pub fn code(&self) -> u16 {
        match self {
            BizCode::Success => 200,
            BizCode::BadRequest => 400,
            BizCode::Unauthorized => 401,
            BizCode::Forbidden => 403,
            BizCode::NotFound => 404,
            BizCode::ServerError => 500,
        }
    }

    // 获取业务码对应的默认消息
    pub fn message(&self) -> &'static str {
        match self {
            BizCode::Success => "操作成功",
            BizCode::BadRequest => "请求参数错误",
            BizCode::Unauthorized => "未授权访问",
            BizCode::Forbidden => "禁止访问",
            BizCode::NotFound => "资源不存在",
            BizCode::ServerError => "服务器内部错误",
        }
    }

    // 获取对应的 HTTP 状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            BizCode::Success => StatusCode::OK,
            BizCode::BadRequest => StatusCode::BAD_REQUEST,
            BizCode::Unauthorized => StatusCode::UNAUTHORIZED,
            BizCode::Forbidden => StatusCode::FORBIDDEN,
            BizCode::NotFound => StatusCode::NOT_FOUND,
            BizCode::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// 统一响应结构体
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,        // 业务状态码
    pub msg: String,      // 消息
    pub data: Option<T>,  // 响应数据

    // 内部字段，不序列化到 JSON
    #[serde(skip)]
    status_code: StatusCode,
}

impl<T> ApiResponse<T> {
    /// 创建一个新的响应
    pub fn new(code: BizCode, msg: Option<&str>, data: Option<T>) -> Self {
        let message = msg.unwrap_or_else(|| code.message()).to_string();

        Self {
            code: code.code(),
            msg: message,
            data,
            status_code: code.status_code(),
        }
    }

    // 成功响应
    pub fn success(data: T) -> Self {
        Self::new(BizCode::Success, None, Some(data))
    }

    // 成功响应，无数据
    pub fn success_empty() -> Self {
        Self::new(BizCode::Success, None, None)
    }

    // 成功响应，自定义消息
    pub fn success_with_msg(data: T, msg: &str) -> Self {
        Self::new(BizCode::Success, Some(msg), Some(data))
    }

    // 错误响应，若不想写msg可传入None
    pub fn error(code: BizCode, msg: Option<&str>) -> Self {
        Self::new(code, msg, None)
    }

    // 设置消息
    pub fn with_msg(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    // 设置数据
    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }
}

// 为 ApiResponse 实现 IntoResponse，使其可以直接作为 Axum 的响应返回
impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}