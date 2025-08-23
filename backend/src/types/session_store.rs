use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rand::Rng;
use std::fmt;

// 会话存储结构体，封装会话管理功能
#[derive(Clone, Default)]
pub struct SessionStore {
    // 使用Arc和Mutex实现线程安全的HashMap存储
    sessions: Arc<Mutex<HashMap<String, String>>>,
}

impl SessionStore {
    // 创建新的会话存储实例
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // 生成随机的会话令牌
    fn generate_token(&self) -> String {
        let mut rng = rand::rng();
        let token: String = (0..32)
            .map(|_| {
                let c = rng.random_range(0..62);
                match c {
                    0..=9 => (b'0' + c as u8) as char,
                    10..=35 => (b'a' + (c - 10) as u8) as char,
                    36..=61 => (b'A' + (c - 36) as u8) as char,
                    _ => unreachable!(),
                }
            })
            .collect();
        token
    }

    // 添加新会话并返回令牌
    pub fn add_session(&self, user_name: &str) -> String {
        let token = self.generate_token();

        // 锁定HashMap并插入新会话
        let mut sessions = self.sessions.lock().expect("Failed to lock session store");
        sessions.insert(token.clone(), user_name.to_string());

        token
    }

    // 根据令牌获取用户名
    pub fn get_user(&self, token: &str) -> Option<String> {
        let sessions = self.sessions.lock().expect("Failed to lock session store");
        sessions.get(token).cloned()
    }

    // 移除会话
    pub fn remove_session(&self, token: &str) -> Option<String> {
        let mut sessions = self.sessions.lock().expect("Failed to lock session store");
        sessions.remove(token)
    }

    // 检查会话是否存在
    pub fn has_session(&self, token: &str) -> bool {
        let sessions = self.sessions.lock().expect("Failed to lock session store");
        sessions.contains_key(token)
    }
}

// 实现Display trait以便于打印
impl fmt::Display for SessionStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sessions = self.sessions.lock().expect("Failed to lock session store");
        write!(f, "SessionStore with {} sessions", sessions.len())
    }
}

