#[test]
fn test_01() {
    // 直接创建+赋值
    let v = vec!["苹果", "香蕉", "橘子", "橙子"];
    for i in v {
        println!("输出是：{}", i);
    }

    // 创建一个空的动态数组 (注意添加 mut 可变 关键词)
    let mut v = Vec::new(); // 如果后面会手动放数据，不需要指定类型
    v.push("value");
    v.push("apple");
    v.push("orange");
    v.push("blanan");

    // 遍历的时候是借用v，否则会丢失所有权
    for &i in &v {
        println!("动态数组中的值是：{}", i);
    }
    v.push("AAAAA");
    for &i in &v {
        println!("动态数组中的值是：{}", i);
    }
}
