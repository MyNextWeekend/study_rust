use once_cell::sync::Lazy;
use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;
use tokio::sync::Mutex;

static POOL: Lazy<Mutex<Option<Pool<MySql>>>> = Lazy::new(|| Mutex::new(None));

pub struct Database;

impl Database {
    // 初始化数据库链接，赋值给全局变量POOL
    pub async fn init() -> Result<(), sqlx::Error> {
        let pool
            = MySqlPoolOptions::new().max_connections(32).connect("mysql://study_python:nTMX8sPDSrsHzLC7@106.55.186.222:3306/study_python").await?;
        let mut pool_guard = POOL.lock().await;
        *pool_guard = Some(pool);
        Ok(())
    }

    // 获取数据库链接池子，私有方法
    async fn get_pool() -> Pool<MySql> {
        let pool_guard = POOL.lock().await;
        pool_guard.as_ref().expect("Database pool is not initialized").clone()
    }

    // 插入sql
    pub async fn insert(name: &str, email: &str) -> Result<i64, sqlx::Error> {
        let pool = Database::get_pool().await;
        let row: (i64,) = sqlx::query_as(
            "insert into user (name,email) values($1,$2)")
            .bind(name).bind(email)
            .fetch_one(&pool).await?;
        Ok(row.0)
    }

    //查询sql
    pub async fn select<T: Send + Unpin + for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow>>() -> Result<Vec<T>, sqlx::Error> {
        let pool = Database::get_pool().await;
        let users = sqlx::query_as("select id,name,email from user").fetch_all(&pool).await?;

        Ok(users)
    }
}


#[cfg(test)]
mod sqlx_test {
    use super::{Database};
    #[derive(Debug, sqlx::FromRow)]
    struct User {
        id: i32,
        name: String,
        email: String,
    }

    #[tokio::test]
    async fn test_01() {
        // 初始化数据库
        Database::init().await.expect("数据库初始化异常:");

        let result = Database::select::<User>().await;
        match result {
            Ok(res) => { println!("查询的结果是:{:?}", res) }
            Err(err) => { println!("收到一个错误:{}", err) }
        }
    }
}