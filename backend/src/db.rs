use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{
    Error,
    sqlite::{SqliteConnectOptions, SqlitePool},
};

const CREATE_TABLES_SQL: &str = include_str!("sql/create_tables.sql");

#[derive(Clone)]
pub struct DbClient {
    pool: SqlitePool,
}

impl DbClient {
    /// 创建数据库连接池并初始化表结构
    pub async fn connect() -> Result<Self, Error> {
        let database_url = "IseKai.db".to_string();

        let options = SqliteConnectOptions::new()
            .filename(database_url)
            .pragma("busy_timeout", "5000") // 遇到锁时等待 5 秒
            .pragma("journal_mode", "WAL") // 启用 WAL 模式提升读并发
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(options)
            .await?;

        // 启用外键支持
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await?;

        Self::init(&pool).await?;

        Ok(Self { pool })
    }

    /// 初始化数据库表结构
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
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== link_groups 表操作 ==============
    pub async fn create_link_group(
        &self,
        user_id: i64,
        name: &str,
        slug: &str,
        key: Option<&str>,
        description: Option<&str>,
        is_public: bool,
        cache_content: Option<&str>,
        cache_refresh_interval: i32,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO link_groups (user_id, name, slug, key, description, is_public, cache_content, cache_refresh_interval)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(name)
            .bind(slug)
            .bind(key)
            .bind(description)
            .bind(is_public)
            .bind(cache_content)
            .bind(cache_refresh_interval)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_link_group_by_id(&self, id: i64) -> Result<LinkGroup, Error> {
        let sql = "SELECT id, user_id, name, slug, key, description, is_public, cache_content,
                          cache_refresh_interval, cache_updated_at, created_at
                   FROM link_groups WHERE id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_groups_by_user(&self, user_id: i64) -> Result<Vec<LinkGroup>, Error> {
        let sql = "SELECT id, user_id, name, slug, key, description, is_public, cache_content,
                          cache_refresh_interval, cache_updated_at, created_at
                   FROM link_groups WHERE user_id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_group_by_slug(&self, slug: &str) -> Result<LinkGroup, Error> {
        let sql = "SELECT id, user_id, name, slug, key, description, is_public, cache_content,
                          cache_refresh_interval, cache_updated_at, created_at
                   FROM link_groups WHERE slug = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(slug)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_link_group(
        &self,
        id: i64,
        name: &str,
        slug: &str,
        key: Option<&str>,
        description: Option<&str>,
        is_public: bool,
        cache_content: Option<&str>,
        cache_refresh_interval: i32,
    ) -> Result<bool, Error> {
        let sql = "UPDATE link_groups
                  SET name = ?, slug = ?, key = ?, description = ?, is_public = ?,
                      cache_content = ?, cache_refresh_interval = ?, cache_updated_at = CURRENT_TIMESTAMP
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(name)
            .bind(slug)
            .bind(key)
            .bind(description)
            .bind(is_public)
            .bind(cache_content)
            .bind(cache_refresh_interval)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update_group_cache(&self, id: i64, cache_content: &str) -> Result<bool, Error> {
        let sql = "UPDATE link_groups
                  SET cache_content = ?, cache_updated_at = CURRENT_TIMESTAMP
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(cache_content)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_link_group(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM link_groups WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== links 表操作 ==============
    pub async fn create_link(
        &self,
        user_id: i64,
        type_: &str,
        is_public: bool,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
        content: &str,
        cache_content: Option<&str>,
        cache_refresh_interval: i32,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO links (user_id, type, is_public, name, slug, description, content, cache_content, cache_refresh_interval)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(type_)
            .bind(is_public)
            .bind(name)
            .bind(slug)
            .bind(description)
            .bind(content)
            .bind(cache_content)
            .bind(cache_refresh_interval)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_link_by_id(&self, id: i64) -> Result<Link, Error> {
        let sql = "SELECT id, user_id, type, is_public, name, slug, description, content,
                          cache_content, cache_refresh_interval, cache_updated_at, created_at
                   FROM links WHERE id = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_link_by_slug(&self, slug: &str) -> Result<Link, Error> {
        let sql = "SELECT id, user_id, type, is_public, name, slug, description, content,
                          cache_content, cache_refresh_interval, cache_updated_at, created_at
                   FROM links WHERE slug = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(slug)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_links_by_user(&self, user_id: i64) -> Result<Vec<Link>, Error> {
        let sql = "SELECT id, user_id, type, is_public, name, slug, description, content,
                          cache_content, cache_refresh_interval, cache_updated_at, created_at
                   FROM links WHERE user_id = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn update_link(
        &self,
        id: i64,
        type_: &str,
        is_public: bool,
        name: Option<&str>,
        slug: Option<&str>,
        description: Option<&str>,
        content: &str,
        cache_content: Option<&str>,
        cache_refresh_interval: i32,
    ) -> Result<bool, Error> {
        let sql = "UPDATE links
                  SET type = ?, is_public = ?, name = ?, slug = ?, description = ?, content = ?,
                      cache_content = ?, cache_refresh_interval = ?, cache_updated_at = CURRENT_TIMESTAMP
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(type_)
            .bind(is_public)
            .bind(name)
            .bind(slug)
            .bind(description)
            .bind(content)
            .bind(cache_content)
            .bind(cache_refresh_interval)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update_link_cache(&self, id: i64, cache_content: &str) -> Result<bool, Error> {
        let sql = "UPDATE links
                  SET cache_content = ?, cache_updated_at = CURRENT_TIMESTAMP
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(cache_content)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_link(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM links WHERE id = ?";
        let result = sqlx::query(sql).bind(id).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== 高级查询操作 ==============
    pub async fn get_public_links(&self) -> Result<Vec<Link>, Error> {
        let sql = "SELECT id, user_id, type, is_public, name, slug, description, content,
                          cache_content, cache_refresh_interval, cache_updated_at, created_at
                   FROM links WHERE is_public = true";
        sqlx::query_as::<_, Link>(sql).fetch_all(&self.pool).await
    }

    pub async fn get_public_groups(&self) -> Result<Vec<LinkGroup>, Error> {
        let sql = "SELECT id, user_id, name, slug, key, description, is_public, cache_content,
                          cache_refresh_interval, cache_updated_at, created_at
                   FROM link_groups WHERE is_public = true";
        sqlx::query_as::<_, LinkGroup>(sql)
            .fetch_all(&self.pool)
            .await
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
    pub name: String,
    pub slug: String,
    pub key: Option<String>,
    pub description: Option<String>,
    pub is_public: bool,
    pub cache_content: Option<String>,
    pub cache_refresh_interval: i32,
    pub cache_updated_at: String,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Link {
    pub id: i64,
    pub user_id: i64,
    #[sqlx(rename = "type")]
    pub type_: String,
    pub is_public: bool,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub content: String,
    pub cache_content: Option<String>,
    pub cache_refresh_interval: i32,
    pub cache_updated_at: String,
    pub created_at: String,
}

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
        let updated = db
            .update_user_password(user_id, "newhash456")
            .await
            .unwrap();
        assert!(updated);

        // 删除用户
        let deleted = db.delete_user(user_id).await.unwrap();
        assert!(deleted);
    }

    #[tokio::test]
    async fn test_link_group_operations() {
        let db = DbClient::connect().await.unwrap();

        // 创建用户
        let user_id = db.create_user("testuser2", "hash123").await.unwrap();

        // 创建链接组
        let group_id = db
            .create_link_group(
                user_id,
                "Test Group",
                "test-group",
                Some("group-key"),
                Some("Test description"),
                true,
                Some("cached content"),
                3600,
            )
            .await
            .unwrap();
        assert!(group_id > 0);

        // 获取链接组
        let group = db.get_link_group_by_id(group_id).await.unwrap();
        assert_eq!(group.name, "Test Group");
        assert_eq!(group.slug, "test-group");

        // 更新链接组
        let updated = db
            .update_link_group(
                group_id,
                "Updated Group",
                "updated-group",
                Some("new-key"),
                Some("Updated description"),
                false,
                Some("updated cache"),
                7200,
            )
            .await
            .unwrap();
        assert!(updated);

        // 删除链接组
        let deleted = db.delete_link_group(group_id).await.unwrap();
        assert!(deleted);

        // 删除用户
        db.delete_user(user_id).await.unwrap();
    }
}
