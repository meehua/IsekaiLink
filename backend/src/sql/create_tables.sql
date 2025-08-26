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

-- 链接组或链接与缓存的关联表（支持一对一关系）
CREATE TABLE IF NOT EXISTS resource_cache (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- 关联类型：'group' 或 'link'
    resource_type TEXT NOT NULL CHECK (resource_type IN ('group', 'link')),
    -- 关联的资源ID（链接组ID或链接ID）
    resource_id INTEGER NOT NULL,
    cache_id INTEGER NOT NULL UNIQUE, -- 确保一对一
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    -- 确保同一类型的同一资源只能关联一个缓存
    UNIQUE(resource_type, resource_id),
    FOREIGN KEY (cache_id) REFERENCES caches(id) ON DELETE CASCADE
    -- 注意：这里不能直接使用外键约束到两个不同的表
    -- 需要在应用层确保 resource_id 的有效性
);

-- 为关联表创建索引以提高查询性能
CREATE INDEX IF NOT EXISTS idx_link_group_association_link ON link_group_association(link_id);
CREATE INDEX IF NOT EXISTS idx_link_group_association_group ON link_group_association(group_id);
CREATE INDEX IF NOT EXISTS idx_resource_cache_type_id ON resource_cache(resource_type, resource_id);
CREATE INDEX IF NOT EXISTS idx_resource_cache_cache ON resource_cache(cache_id);