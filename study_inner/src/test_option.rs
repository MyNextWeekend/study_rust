fn do_some_thing() {
    // unwrap()方法
    //unwrap()方法用于获取Option<T>中的值，如果Option<T>是None，则会引发panic。
    let some_number = Some(7);
    let number = some_number.unwrap(); // 有值返回值，无值则panic
    let number = some_number.unwrap_or(99); // 有值返回值，无值则返回99
    let number = some_number.unwrap_or_else(|| 99); // 有值返回值，无值则返回99
    let number = some_number.unwrap_or_default(); // 有值返回值，无值则返回默认值

    // option 转 option
    let some_number = Some(7);
    let some_number = some_number.or(Some(99)); // 有值返回值，无值则返回99
    let some_number = some_number.or_else(|| Some(99)); // 有值返回值，无值则返回99
    let some_number = some_number.and(Some("好好学习，天天向上".to_owned())); // 有值其他类型返回值，无值则返回None
    let some_number = some_number.and_then(|x| Some(x.to_string())); // 有值其他类型返回值，无值则返回None

    // map()方法
    //map()方法用于对Option<T>中的值进行转换，如果Option<T>是None，则返回None。
    let some_number = Some(7);
    let other_number = some_number.map(|x| x + 1); // 有值返回值，无值则返回None
    let other_number = some_number.map_or(99, |x| x + 1); // 有值返回值，无值则返回99
    let other_number = some_number.map_or_else(|| 99, |x| x + 1); // 有值返回值，无值则返回99

    // option 转 result
    //ok_or()方法用于将Option<T>转换为Result<T, E>，如果Option<T>是Some(T)，则返回Ok(T)，否则返回Err(E)。
    let some_number = Some(7);
    let other_number = some_number.ok_or("没有值".to_string()); // 有值返回值，无值则返回Err
    let other_number = some_number.ok_or_else(|| "没有值".to_string()); // 有值返回值，无值则返回Err

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
