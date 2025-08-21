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
    group_name TEXT NOT NULL,
    group_slug TEXT NOT NULL UNIQUE,
    group_key TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 缓存表
CREATE TABLE IF NOT EXISTS file_caches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    cache_slug TEXT UNIQUE,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 具体链接表
CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    group_id INTEGER NOT NULL,
    cache_id INTEGER, -- 可以为NULL，表示没有独立缓存
    type TEXT NOT NULL,
    url TEXT NOT NULL,
    name TEXT,
    content TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES link_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (cache_id) REFERENCES file_caches(id) ON DELETE CASCADE
);

-- 链接组与缓存的关联表（实现一对一关系）
CREATE TABLE IF NOT EXISTS group_cache (
    group_id INTEGER NOT NULL UNIQUE, -- 确保一对一
    cache_id INTEGER NOT NULL UNIQUE, -- 确保一对一
    PRIMARY KEY (group_id, cache_id),
    FOREIGN KEY (group_id) REFERENCES link_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (cache_id) REFERENCES file_caches(id) ON DELETE CASCADE
);