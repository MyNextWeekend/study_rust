// 泛型学习

/// 泛型方法
fn largest<T>(list: &[T]) -> Result<T, String>
where
    T: Ord + Copy,
{
    if list.len() == 0 {
        return Err("not enough arguments".into());
    }
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    Ok(largest)
}


/// 泛型结构体
struct Proper<T> {
    name: String,
    age: i32,
    hobby: T,
}

impl<T> Proper<T> {
    fn new(hobby: T) -> Self {
        Proper {
            name: String::from("value"),
            age: 18,
            hobby,
        }
    }
}


#[cfg(test)]
mod generic_test {
    use super::*;

    #[test]
    fn test_01() {
        let a = [2, 4, 6, 3, 1];
        let b: Vec<i32> = Vec::new();
        println!("max = {:?}", largest(&a));
        println!("max = {:?}", largest(&b));

        let user1 = Proper::new(vec!["唱".to_string(), "跳".to_string(), "rap".to_string()]);
        let user2: Proper<String> = Proper::new("玩游戏".into());
        println!("user1 name:{},age:{},hobby:{:?}", user1.name, user1.age, user1.hobby);
        println!("user2 name:{},age:{},hobby:{:?}", user2.name, user2.age, user2.hobby);
    }
}

