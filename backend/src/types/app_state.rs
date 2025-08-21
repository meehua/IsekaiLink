use crate::db::DbClient;
use crate::types::session_store::SessionStore;
// 应用状态
#[derive(Clone)]
pub struct AppState {
    pub db_client: DbClient, // 数据库
    pub sessions: SessionStore, // 内存 Session 存储
}
