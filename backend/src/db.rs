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
        is_public: bool,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO link_groups (user_id, group_name, group_slug, group_key, is_public)
                   VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(group_name)
            .bind(group_slug)
            .bind(group_key)
            .bind(is_public)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_link_group_by_id(&self, id: i64) -> Result<LinkGroup, Error> {
        let sql = "SELECT id, user_id, group_name, group_slug, group_key, is_public, created_at
                   FROM link_groups WHERE id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn get_groups_by_user(&self, user_id: i64) -> Result<Vec<LinkGroup>, Error> {
        let sql = "SELECT id, user_id, group_name, group_slug, group_key, is_public, created_at
                   FROM link_groups WHERE user_id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_group_by_slug(&self, slug: &str) -> Result<LinkGroup, Error> {
        let sql = "SELECT id, user_id, group_name, group_slug, group_key, is_public, created_at
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
        is_public: bool,
    ) -> Result<bool, Error> {
        let sql = "UPDATE link_groups
                  SET group_name = ?, group_slug = ?, group_key = ?, is_public = ?
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_name)
            .bind(new_slug)
            .bind(new_key)
            .bind(is_public)
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
        user_id: i64,
        link_type: Option<&str>,
        url: &str,
        name: Option<&str>,
        content: Option<&str>,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO links (user_id, type, url, name, content)
                   VALUES (?, ?, ?, ?, ?)";
        let result = sqlx::query(sql)
            .bind(user_id)
            .bind(link_type)
            .bind(url)
            .bind(name)
            .bind(content)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_link_by_id(&self, id: i64) -> Result<Link, Error> {
        let sql = "SELECT id, user_id, type, url, name, content, created_at
                   FROM links WHERE id = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }


    pub async fn get_links_by_user(&self, user_id: i64) -> Result<Vec<Link>, Error> {
        let sql = "SELECT id, user_id, type, url, name, content, created_at
                   FROM links WHERE user_id = ?";
        sqlx::query_as::<_, Link>(sql)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
    }


    pub async fn get_links_by_group(&self, group_id: i64) -> Result<Vec<Link>, Error> {
        let sql = "SELECT l.id, l.user_id, l.type, l.url, l.name, l.content, l.created_at
                   FROM links l
                   JOIN link_group_association lga ON l.id = lga.link_id
                   WHERE lga.group_id = ?";
        sqlx::query_as::<_, Link>(sql)
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
        new_type: Option<&str>,
    ) -> Result<bool, Error> {
        let sql = "UPDATE links
                  SET url = ?, name = ?, content = ?, type = ?
                  WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_url)
            .bind(new_name)
            .bind(new_content)
            .bind(new_type)
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

    // ============== caches 表操作 ==============
    pub async fn create_cache(
        &self,
        content: &str,
        refresh_interval: i32,
    ) -> Result<i64, Error> {
        let sql = "INSERT INTO caches (content, refresh_interval) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(content)
            .bind(refresh_interval)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get_cache_by_id(&self, id: i64) -> Result<Cache, Error> {
        let sql = "SELECT id, content, refresh_interval, created_at, updated_at
                   FROM caches WHERE id = ?";
        sqlx::query_as::<_, Cache>(sql)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn update_cache_content(
        &self,
        id: i64,
        new_content: &str,
    ) -> Result<bool, Error> {
        let sql = "UPDATE caches SET content = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(new_content)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_cache(&self, id: i64) -> Result<bool, Error> {
        let sql = "DELETE FROM caches WHERE id = ?";
        let result = sqlx::query(sql)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== link_group_association 表操作 ==============
    // 链接与组的关联操作
    pub async fn add_link_to_group(
        &self,
        link_id: i64,
        group_id: i64,
    ) -> Result<bool, Error> {
        let sql = "INSERT OR IGNORE INTO link_group_association (link_id, group_id) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(link_id)
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn remove_link_from_group(
        &self,
        link_id: i64,
        group_id: i64,
    ) -> Result<bool, Error> {
        let sql = "DELETE FROM link_group_association WHERE link_id = ? AND group_id = ?";
        let result = sqlx::query(sql)
            .bind(link_id)
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_groups_for_link(&self, link_id: i64) -> Result<Vec<LinkGroup>, Error> {
        let sql = "SELECT lg.id, lg.user_id, lg.group_name, lg.group_slug, lg.group_key, lg.is_public, lg.created_at
                   FROM link_groups lg
                   JOIN link_group_association lga ON lg.id = lga.group_id
                   WHERE lga.link_id = ?";
        sqlx::query_as::<_, LinkGroup>(sql)
            .bind(link_id)
            .fetch_all(&self.pool)
            .await
    }

    // ============== 组缓存关联操作 ==============
    pub async fn create_group_cache_association(
        &self,
        group_id: i64,
        cache_id: i64,
    ) -> Result<bool, Error> {
        let sql = "INSERT OR REPLACE INTO group_caches (group_id, cache_id) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(group_id)
            .bind(cache_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_cache_for_group(
        &self,
        group_id: i64,
    ) -> Result<Option<Cache>, Error> {
        let sql = "SELECT c.id, c.content, c.refresh_interval, c.created_at, c.updated_at
                   FROM caches c
                   JOIN group_caches gc ON c.id = gc.cache_id
                   WHERE gc.group_id = ?";
        sqlx::query_as::<_, Cache>(sql)
            .bind(group_id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn delete_group_cache_association(
        &self,
        group_id: i64,
    ) -> Result<bool, Error> {
        let sql = "DELETE FROM group_caches WHERE group_id = ?";
        let result = sqlx::query(sql)
            .bind(group_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== 链接缓存关联操作 ==============
    pub async fn create_link_cache_association(
        &self,
        link_id: i64,
        cache_id: i64,
    ) -> Result<bool, Error> {
        let sql = "INSERT OR REPLACE INTO link_caches (link_id, cache_id) VALUES (?, ?)";
        let result = sqlx::query(sql)
            .bind(link_id)
            .bind(cache_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn get_cache_for_link(
        &self,
        link_id: i64,
    ) -> Result<Option<Cache>, Error> {
        let sql = "SELECT c.id, c.content, c.refresh_interval, c.created_at, c.updated_at
                   FROM caches c
                   JOIN link_caches lc ON c.id = lc.cache_id
                   WHERE lc.link_id = ?";
        sqlx::query_as::<_, Cache>(sql)
            .bind(link_id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn delete_link_cache_association(
        &self,
        link_id: i64,
    ) -> Result<bool, Error> {
        let sql = "DELETE FROM link_caches WHERE link_id = ?";
        let result = sqlx::query(sql)
            .bind(link_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    // ============== 高级查询操作 ==============
    pub async fn get_group_with_links_and_cache(&self, group_id: i64) -> Result<GroupWithDetails, Error> {
        // 获取组信息
        let group = self.get_link_group_by_id(group_id).await?;

        // 获取组的所有链接
        let links = self.get_links_by_group(group_id).await?;

        // 获取组的缓存关联
        let cache = self.get_cache_for_group(group_id).await?;

        Ok(GroupWithDetails {
            group,
            links,
            cache,
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
    pub group_slug: String,
    pub group_key: Option<String>,
    pub is_public: bool,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Link {
    pub id: i64,
    pub user_id: i64,
    #[sqlx(rename = "type")]
    pub type_: Option<String>,
    pub url: String,
    pub name: Option<String>,
    pub content: Option<String>,
    pub created_at: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Cache {
    pub id: i64,
    pub content: String,
    pub refresh_interval: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug)]
pub struct GroupWithDetails {
    pub group: LinkGroup,
    pub links: Vec<Link>,
    pub cache: Option<Cache>,
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
        let updated = db.update_user_password(user_id, "newhash456").await.unwrap();
        assert!(updated);

        // 删除用户
        let deleted = db.delete_user(user_id).await.unwrap();
        assert!(deleted);
    }
}