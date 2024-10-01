use dotenv::dotenv;
use std::env;

#[cfg(test)]
mod dotenv_test {
    use super::*;
    #[test]
    fn test_01() {
        // 加载 .env 文件
        dotenv().ok();

        // 获取环境变量
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let api_key = env::var("API_KEY").expect("API_KEY must be set");

        // 使用环境变量
        println!("Connecting to database at: {}", database_url);
        println!("Using API key: {}", api_key);
    }
}