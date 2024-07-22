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

    // 插入sql，这里不能做到通用，一个sql一个方法
    pub async fn insert_user(name: &str, email: &str) -> Result<i64, sqlx::Error> {
        let pool = Database::get_pool().await;
        let row: (i64,) = sqlx::query_as(
            "insert into user (name,email) values($1,$2)")
            .bind(name).bind(email)
            .fetch_one(&pool).await?;
        Ok(row.0)
    }

    //查询sql
    pub async fn get_user() -> Result<Vec<User>, sqlx::Error> {
        let pool = Database::get_pool().await;
        let users: Vec<User> = sqlx::query_as(
            "select id,name,email from user").fetch_all(&pool).await?;
        Ok(users)
    }
}

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[cfg(test)]
mod sqlx_test {
    use crate::study_other::test_sqlx::{Database};

    #[tokio::test]
    async fn test_01() -> Result<(), sqlx::Error> {
        // 初始化数据库
        Database::init().await?;

        match Database::get_user().await {
            Ok(users) => { println!("查询的结果是：{:?}", users) }
            Err(e) => { println!("出现异常是：{:?}", e) }
        }
        Ok(())
    }
}