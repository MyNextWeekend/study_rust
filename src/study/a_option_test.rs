#[test]
fn test_01() {
    let some_number = Some(5);
    let some_str = Some(String::from("value"));
    let some_str2 = Some("nihao");
    // 将None赋值给一个option类型的变量的时候，需要显示指明option的类型
    let some_null: Option<String> = None;

    //使用option转换为普通类型的时候
    match some_str2 {
        Some(value) => println!("获取的值是：{value}"),
        None => println!("是一个空值"),
    };
}
