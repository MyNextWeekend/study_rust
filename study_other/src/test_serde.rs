use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Proper {
    id: Option<usize>,
    #[serde(rename = "userName")] // 此处可以让struct字段与json字段匹配
    user_name: String,
    age: Option<usize>,
    //避免序列化的时候没有这个值，所以用Option包裹
    hobbies: Vec<String>,
}

impl Proper {
    fn new() -> Self {
        Proper {
            id: Some(1),
            user_name: String::from("章三"),
            age: Some(18),
            hobbies: vec!["唱".to_string(), "跳".to_string(), "rap".to_string()],
        }
    }
}

#[cfg(test)]
mod serde_test {
    use super::*;

    #[test]
    fn test_01() {
        let user = Proper::new();
        let user = serde_json::to_string(&user);
        match user {
            Ok(u) => println!("序列化之后得到:{}", u), //{"id":1,"name":"章三","age":18,"hobbies":["唱","跳","rap"]}
            Err(err) => print!("序列化失败:{}", err.to_string()),
        }
        println!("============================");

        let user_str = r#"{"userName":"章三","age":18,"hobbies":["唱","跳","rap"]}"#;
        let user: Result<Proper, serde_json::Error> = serde_json::from_str(user_str);
        match user {
            Ok(u) => {
                println!("反序列化之后得到:{:?}", u); //Proper { id: None, user_name: "章三", age: Some(18), hobbies: ["唱", "跳", "rap"] }
                match u.id {
                    Some(id) => println!("id的值是:{}", id),
                    None => println!("id的值是:None"), //id的值是:None
                };
            }
            Err(err) => println!("反序列化异常:{}", err.to_string()),
        };
    }
}


