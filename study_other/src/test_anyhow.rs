use std::collections::HashMap;
// 主要用于非lib项目中
use anyhow::Result;

fn get_cluster_info() -> Result<HashMap<String, String>> {
    let config = std::fs::read_to_string("cluster.json")?;
    let map: HashMap<String, String> = serde_json::from_str(&config)?;
    Ok(map)
}

#[cfg(test)]
mod anyhow_test{
    use super::*;

    #[test]
    fn test01(){
        let result = get_cluster_info();
        match result {
            Ok(res) => {println!("结果是:{}",res.len())}
            Err(err) => {println!("异常是:{}",err)}
        }
    }
}