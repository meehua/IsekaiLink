use std::time::Duration;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePool},
    Error
};
use sqlx::sqlite::SqlitePoolOptions;

const CREATE_TABLES_SQL: &str = include_str!("sql/create_tables.sql");

#[derive(Clone)]
pub struct DbClient {
    pool: SqlitePool,
}

// 构建 SQLite 连接池

impl DbClient {
    /// 创建数据库连接池并初始化表结构
    pub async fn connect() -> Result<Self, Error> {
        let database_url = "IseKai.db".to_string();

        let options = SqliteConnectOptions::new()
            .filename(database_url)
            .pragma("busy_timeout", "5000")  // 遇到锁时等待 5 秒
            .pragma("journal_mode", "WAL")  // 启用 WAL 模式提升读并发
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)  // 根据压测并发量调整（不宜过大，SQLite 并发能力有限）
            .connect_with(options)
            .await?;

        // 启用外键支持
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await?;

        Self::init(&pool).await?;

        Ok(Self { pool })
    }

    /// 初始化数据库表结构，辅助函数
    async fn init(pool: &SqlitePool) -> Result<(), Error> {
        sqlx::query(CREATE_TABLES_SQL)
            .execute(pool)
            .await
            .expect("Failed to initialize database tables");

        Ok(())
    }

    /// 获取数据库连接池引用
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 执行原始SQL查询
    pub async fn execute_raw(&self, sql: &str) -> Result<u64, Error> {
        let result = sqlx::query(sql)
            .execute(&self.pool)
            .await
            .expect("Failed to execute raw SQL");

        Ok(result.rows_affected())
    }

    // ============== users 表操作 ==============
    pub async fn create_user(&self, username: &str, pwd_hash: &str) -> Result<i64, Error> {
        let sql = "INSERT INTO users (username, pwd_hash) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(username)
            .bind(pwd_hash)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<User, Error> {
        let sql = "SELECT id, username, pwd_hash, created_at FROM users WHERE id = ?";
        sqlx::query_as::<_, User>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, Error> {
        let sql = "SELECT id, username, pwd_hash, created_at FROM users WHERE username = ?";
        sqlx::query_as::<_, User>(sql)
            .bind(username)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_user_password(&self, id: i64, new_pwd_hash: &str) -> Result<bool, Error> {
        let sql = "UPDATE users SET pwd_hash = ? WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_pwd_hash)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_user(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM users WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== link_groups 表操作 ==============
    pub async fn create_link_group(
        &self,
        user_id: i64,
        group_name: &str,
        group_slug: &str,
        group_key: Option<&str>,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO link_groups (user_id, group_name, group_slug, group_key)
                   VALUES (?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(group_name)
            .bind(group_slug)
            .bind(group_key)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_link_group_by_id(&self, id: i64) -> Result<LinkGroup, Error> {
        let sql = "SELECT id, user_id, group_name, group_slug, group_key, created_at
                   FROM link_groups WHERE id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_groups_by_user(&self, user_id: i64) -> Result<Vec<LinkGroup>, Error> {
        let sql = "SELECT id, user_id, group_name, group_slug, group_key, created_at
                   FROM link_groups WHERE user_id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_group_by_slug(&self, slug: &str) -> Result<LinkGroup, Error> {
        let sql = "SELECT id, user_id, group_name, group_slug, group_key, created_at
                   FROM link_groups WHERE group_slug = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(slug)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_link_group(
        &self,
        id: i64,
        new_name: &str,
        new_slug: &str,
        new_key: Option<&str>,
    ) -> Result<bool, Error> {
        let sql = "UPDATE link_groups
                  SET group_name = ?, group_slug = ?, group_key = ?
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_name)
            .bind(new_slug)
            .bind(new_key)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_link_group(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM link_groups WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== links 表操作 ==============
    pub async fn create_link(
        &self,
        group_id: i64,
        link_type: &str,
        url: &str,
        name: Option<&str>,
        content: Option<&str>,
        cache_id: Option<i64>,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO links (group_id, type, url, name, content, cache_id)
                   VALUES (?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(group_id)
            .bind(link_type)
            .bind(url)
            .bind(name)
            .bind(content)
            .bind(cache_id)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_link_by_id(&self, id: i64) -> Result<Link, Error> {
        let sql = "SELECT id, group_id, cache_id, type, url, name, content, created_at
                   FROM links WHERE id = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_links_by_group(&self, group_id: i64) -> Result<Vec<Link>, Error> {
        let sql = "SELECT id, group_id, cache_id, type, url, name, content, created_at
                   FROM links WHERE group_id = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(group_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_links_with_cache_info(&self, group_id: i64) -> Result<Vec<LinkWithCache>, Error> {
        let sql = "
            SELECT
                l.id, l.group_id, l.cache_id, l.type, l.url, l.name, l.content, l.created_at,
                fc.cache_slug, fc.content as cache_content
            FROM links l
            LEFT JOIN file_caches fc ON l.cache_id = fc.id
            WHERE l.group_id = ?
        ";
        sqlx::query_as::<_, LinkWithCache>(sql)
            .bind(group_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update_link(
        &self,
        id: i64,
        new_url: &str,
        new_name: Option<&str>,
        new_content: Option<&str>,
        new_cache_id: Option<i64>,
    ) -> Result<bool, Error> {
        let sql = "UPDATE links
                  SET url = ?, name = ?, content = ?, cache_id = ?
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_url)
            .bind(new_name)
            .bind(new_content)
            .bind(new_cache_id)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_link(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM links WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== file_caches 表操作 ==============
    pub async fn create_file_cache(
        &self,
        cache_slug: Option<&str>,
        content: &str,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO file_caches (cache_slug, content)
                   VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(cache_slug)
            .bind(content)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_file_cache_by_id(&self, id: i64) -> Result<FileCache, Error> {
        let sql = "SELECT id, cache_slug, content, created_at, updated_at
                   FROM file_caches WHERE id = ?";
        sqlx::query_as::<_, FileCache>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_file_cache_by_slug(&self, cache_slug: &str) -> Result<FileCache, Error> {
        let sql = "SELECT id, cache_slug, content, created_at, updated_at
                   FROM file_caches WHERE cache_slug = ?";
        sqlx::query_as::<_, FileCache>(sql)
            .bind(cache_slug)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_file_cache_content(
        &self,
        id: i64,
        new_content: &str,
    ) -> Result<bool, Error> {
        let sql = "UPDATE file_caches SET content = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_content)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_file_cache(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM file_caches WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== group_cache 关联表操作 ==============
    pub async fn create_group_cache_association(
        &self,
        group_id: i64,
        cache_id: i64,
    ) -> Result<bool, Error> {
        let sql = "INSERT INTO group_cache (group_id, cache_id) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(group_id)
            .bind(cache_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_group_cache_association(&self, group_id: i64) -> Result<GroupCacheAssociation, Error> {
        let sql = "SELECT group_id, cache_id FROM group_cache WHERE group_id = ?";
        sqlx::query_as::<_, GroupCacheAssociation>(sql)
            .bind(group_id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn delete_group_cache_association(&self, group_id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM group_cache WHERE group_id = ?";
        let result = sqlx::query(sql)
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== 高级查询操作 ==============
    pub async fn get_group_with_links_and_cache(&self, group_id: i64) -> Result<GroupWithDetails, Error> {
        // 获取组信息
        let group = self.get_link_group_by_id(group_id).await?;

        // 获取组的所有链接
        let links = self.get_links_with_cache_info(group_id).await?;

        // 获取组的缓存关联
        let cache_association = match self.get_group_cache_association(group_id).await {
            Ok(assoc) => Some(assoc),
            Err(_) => None, // 如果没有关联缓存，忽略错误
        };

        // 获取缓存内容（如果有）
        let cache_content = if let Some(assoc) = &cache_association {
            match self.get_file_cache_by_id(assoc.cache_id).await {
                Ok(cache) => Some(cache.content),
                Err(_) => None,
            }
        } else {
            None
        };

        Ok(GroupWithDetails {
            group,
            links,
            cache_association,
            cache_content,
        })
    }
}

// 定义返回类型的结构体
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub pwd_hash: String,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LinkGroup {
    pub id: i64,
    pub user_id: i64,
    pub group_name: String,
    pub group_slug: String, // 不再为Option，因为数据库设计为NOT NULL
    pub group_key: Option<String>,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Link {
    pub id: i64,
    pub group_id: i64,
    pub cache_id: Option<i64>, // 新增字段
    #[sqlx(rename = "type")]
    pub type_: String,
    pub url: String,
    pub name: Option<String>,
    pub content: Option<String>,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct LinkWithCache {
    pub id: i64,
    pub group_id: i64,
    pub cache_id: Option<i64>,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub url: String,
    pub name: Option<String>,
    pub content: Option<String>,
    pub created_at: String,
    pub cache_slug: Option<String>,
    pub cache_content: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct FileCache {
    pub id: i64,
    pub cache_slug: Option<String>,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct GroupCacheAssociation {
    pub group_id: i64,
    pub cache_id: i64,
}

// 高级查询结果结构体
#[derive(Debug)]
pub struct GroupWithDetails {
    pub group: LinkGroup,
    pub links: Vec<LinkWithCache>,
    pub cache_association: Option<GroupCacheAssociation>,
    pub cache_content: Option<String>,
}

// 单元测试示例
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_user_operations() {
        let db = DbClient::connect().await.unwrap();

        // 创建用户
        let user_id = db.create_user("testuser", "hash123").await.unwrap();
        assert!(user_id > 0);

        // 获取用户
        let user = db.get_user_by_id(user_id).await.unwrap();
        assert_eq!(user.username, "testuser");

        // 通过用户名获取用户
        let user_by_name = db.get_user_by_username("testuser").await.unwrap();
        assert_eq!(user_by_name.id, user_id);

        // 更新密码
        let updated = db.update_user_password(user_id, "newhash456").await.unwrap();
        assert!(updated);

        // 删除用户
        let deleted = db.delete_user(user_id).await.unwrap();
        assert!(deleted);
    }
}