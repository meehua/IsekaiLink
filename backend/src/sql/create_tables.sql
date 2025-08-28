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
    group_name TEXT,
    group_slug TEXT NOT NULL UNIQUE,
    group_key TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 缓存表
CREATE TABLE IF NOT EXISTS caches (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT,
    refresh_interval INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 具体链接表
CREATE TABLE IF NOT EXISTS links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    type TEXT, -- 可以为NULL，表示普通链接
    url TEXT NOT NULL,
    name TEXT,
    content TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- 链接与链接组的关联表（多对多关系）
CREATE TABLE IF NOT EXISTS link_group_association (
    link_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (link_id, group_id),
    FOREIGN KEY (link_id) REFERENCES links(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES link_groups(id) ON DELETE CASCADE
);

-- 链接组缓存关联表
CREATE TABLE IF NOT EXISTS group_caches (
    group_id INTEGER NOT NULL PRIMARY KEY, -- 一对一关系，每个组最多一个缓存
    cache_id INTEGER NOT NULL UNIQUE,      -- 确保缓存唯一性
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES link_groups(id) ON DELETE CASCADE,
    FOREIGN KEY (cache_id) REFERENCES caches(id) ON DELETE CASCADE
);

-- 链接缓存关联表
CREATE TABLE IF NOT EXISTS link_caches (
    link_id INTEGER NOT NULL PRIMARY KEY, -- 一对一关系，每个链接最多一个缓存
    cache_id INTEGER NOT NULL UNIQUE,     -- 确保缓存唯一性
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (link_id) REFERENCES links(id) ON DELETE CASCADE,
    FOREIGN KEY (cache_id) REFERENCES caches(id) ON DELETE CASCADE
);

-- 为关联表创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_link_group_association_link ON link_group_association(link_id);
CREATE INDEX IF NOT EXISTS idx_link_group_association_group ON link_group_association(group_id);
CREATE INDEX IF NOT EXISTS idx_group_caches_cache ON group_caches(cache_id);
CREATE INDEX IF NOT EXISTS idx_link_caches_cache ON link_caches(cache_id);