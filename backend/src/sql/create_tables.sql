PRAGMA foreign_keys = ON;

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    pwd_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 链接组表
CREATE TABLE IF NOT EXISTS link_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    name TEXT,
    slug TEXT UNIQUE,  -- 该链接组的唯一标识符（如果对外开放的话）
    key TEXT,  -- 该链接组的访问密钥（如果对外开放的话）
    description TEXT,  -- 一些描述与说明
    is_public BOOLEAN DEFAULT FALSE,  -- 该链接组是否公开
    cache_content TEXT,
    cache_refresh_interval INTEGER DEFAULT 0,
    cache_updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 具体链接表
CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    type TEXT NOT NULL,  -- 链接类型，比如 "clash", "norm（普通链接，用于缓存）"等
    is_public BOOLEAN DEFAULT FALSE,  -- 该链接是否公开(如果是独立链接的话)
    name TEXT,
    slug TEXT UNIQUE,  -- 该链接的唯一标识符(如果是独立链接的话)
    description TEXT,
    content TEXT,  -- 链接内容，是URL
    cache_content TEXT,  -- 缓存内容（如果是独立链接的话）
    cache_refresh_interval INTEGER DEFAULT 0,  -- 缓存刷新间隔，单位为秒
    cache_updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,  -- 上次缓存更新时间
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
