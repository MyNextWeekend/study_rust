// Reqwest第三方包学习

use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//同步请求
#[test]
fn test_01() {
    let url = "https://httpbin.org/ip".to_string();
    let res = request(url).unwrap();
    println!("收到数据: {}", res)
}

fn request(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(url)?.text()?;
    println!("拿到结果是: {}", resp);
    Ok(resp)
}

//异步请求 async
#[tokio::main] //声明异步开始的位置
#[test]
async fn test_02() {
    let url = "https://httpbin.org/ip".to_string();
    let res = async_request(&url).await;
    match res {
        Ok(res) => println!("收到数据: {:#?}", res),
        Err(err) => println!("异常了呀: {}", err.to_string()),
    }

    let res = async_request_struct(&url).await;
    match res {
        Ok(res) => println!("收到数据: {:?}", res),
        Err(err) => println!("异常了呀: {}", err.to_string()),
    }
}

// 异步请求得到HashMap
async fn async_request(url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new(); //可以自定义客户端header，重复利用
    let resp = client
        .get(url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("拿到结果是: {:#?}", resp);
    Ok(resp)
}

// 声明实现了序列化和反序列化以及格式化输出
#[derive(Debug, Deserialize, Serialize)]
struct Posts {
    origin: String,
}

// 异步请求得到HashMap
async fn async_request_struct(url: &str) -> Result<Posts, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new(); //可以自定义客户端header，重复利用
    let resp = client.get(url).send().await?.json::<Posts>().await?;
    println!("拿到结果是: {:?}", resp);
    Ok(resp)
}
