use std::collections::HashMap;

fn do_some_thing() {
    // 创建一个可变的map
    let mut map = HashMap::new();
    map.insert(String::from("k1"), 1);
    map.insert(String::from("k2"), 2);
    map.entry(String::from("k3")).or_insert(3); // map中没有 k3 的时候才插入，他的值为 3
    map.entry(String::from("k3")).or_insert(33); // map中没有 k3 的时候才插入，他的值为 3

    //因为 get 返回 Option<V>，所以结果被装进 Some；如果某个键在哈希 map 中没有对应的值，get 会返回 None
    let _value = map.get("k3");

    // 如果map中没有 k5 就插入，值为0
    let k5_value = map.entry(String::from("k5")).or_insert(0);
    *k5_value += 1; // 对值进行 +1 操作

    // 遍历map的时候需要借用
    for (key, value) in &map {
        println!("map中的值是 {}: {} ", key, value);
    }
}


#[cfg(test)]
mod map_test {
    use super::*;

    #[test]
    fn test_01() {
        do_some_thing();
    }
}

