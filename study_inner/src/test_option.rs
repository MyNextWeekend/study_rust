fn do_some_thing() {
    // 将None赋值给一个option类型的变量的时候，需要显示指明option的类型
    let some_null = None;
    // 如果选项包含一个值，则返回该选项，否则返回提供的值
    let option: Option<String> = some_null.or(Some("996".into()));
    assert_eq!(option, Some("996".into()));


    //如果值为None，则将其插入到选项中，然后返回对所包含值的可变引用。
    let mut some_number = None;
    let x = some_number.get_or_insert(7);
    assert_eq!(x, &7);
    *x = 996;
    assert_eq!(some_number, Some(996));


    //使用option转换为普通类型的时候
    match Some::<String>("好好学习，天天向上".into()) {
        Some(value) => println!("获取的值是：{value}"),
        None => println!("是一个空值"),
    };
}


#[cfg(test)]
mod option_test {
    use super::*;
    #[test]
    fn test_01() {
        do_some_thing();
    }
}
