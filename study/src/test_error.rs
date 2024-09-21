fn is_even(number: i32) -> Result<bool, String> {
    if number % 2 == 0 {
        Ok(true)
    } else {
        let res = format!("输入的值:{} ,不是偶数", number);
        Err(res)
    }
}

#[cfg(test)]
mod error_test {
    use crate::test_error::is_even;

    #[test]
    #[should_panic]
    fn test_01() {
        // unwrap 是Result<T,E>的方法，
        // 调用此方法的时候，如果是OK，返回OK中的对象，如果是Err枚举，在运行时会panic
        let result = is_even(6).unwrap();
        println!("结果是:{}", result);

        // expect区别是：可以自定义添加一些报错信息
        let result = is_even(7).expect("输入的有误");
        println!("结果是:{}", result);
    }
}
