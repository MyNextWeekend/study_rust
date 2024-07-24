// Reqwest第三方包学习

use std::collections::HashMap;
use serde::de::DeserializeOwned;

struct HttpClient;

impl HttpClient {
    // 阻塞请求
    pub fn get_block(url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let resp = reqwest::blocking::get(url)?.text()?;
        println!("拿到结果是: {}", resp);
        Ok(resp)
    }
    // 异步get请求
    pub async fn get(url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new(); //可以自定义客户端，重复利用
        let resp = client
            .get(url)
            .send()
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("拿到结果是: {:#?}", resp);
        Ok(resp)
    }

    // 异步post请求
    pub async fn post<T: std::fmt::Debug+ DeserializeOwned>(url: &str, body: &HashMap<String, String>) -> Result<T, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new(); //可以自定义客户端，重复利用
        let resp = client
            .post(url)
            .header("Content-Type".to_string(), "application/json".to_string())
            .json(&body)
            .send()
            .await?
            .json::<T>()
            .await?;
        println!("拿到结果是: {:?}", resp);
        Ok(resp)
    }
}

#[cfg(test)]
mod reqwest_test {
    use std::collections::HashMap;
    use crate::study_other::test_reqwest::HttpClient;

    //同步请求
    #[test]
    fn test_01() {
        let url = "https://httpbin.org/ip";
        let res = HttpClient::get_block(url).expect("同步请求失败");
        println!("收到数据: {}", res)
    }


    //异步请求 async
    #[tokio::test]
    async fn test_02() {
        let url = "https://httpbin.org/ip";
        let res = HttpClient::get(url).await;
        match res {
            Ok(res) => println!("收到数据: {:#?}", res),
            Err(err) => println!("异常了呀: {}", err.to_string()),
        }
    }

    #[tokio::test]
    async fn test_03() {
        let url = "http://httpbin.org/post";
        let mut data = HashMap::new();
        data.insert("file_content".into(), "aaa".into());
        data.insert("channel_num".into(), "CCTV".into());
        data.insert("file_id".into(), "98997".into());
        data.insert("file_type".into(), "jog".into());
        data.insert("file_size".into(), "45646".into());
        let res = HttpClient::post::<HashMap<String, String>>(&url, &data).await;
        match res {
            Ok(res) => println!("收到数据: {:?}", res),
            Err(err) => println!("异常了呀: {}", err.to_string()),
        }
    }
}

