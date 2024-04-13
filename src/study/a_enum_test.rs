#[cfg(test)]
mod enum_test {
    // 方式一：结构体中直接写字段名称
    #[derive(Debug)]
    enum IpAddr {
        V4,
        V6,
    }

    // 方式二：将数据附加到枚举的变体中（利用struct）
    #[derive(Debug)]
    struct IpAddrWithStr {
        kind: IpAddr,
        address: String,
    }


    #[test]
    fn test_01() {
        let home = IpAddrWithStr {
            kind: IpAddr::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddrWithStr {
            kind: IpAddr::V6,
            address: String::from("::1"),
        };
        println!("ip{:?}端口是:{}", home.kind, home.address);
        println!("ip{:?}端口是:{}", loopback.kind, loopback.address);
    }

    // 方式三：将数据附加到枚举的变体中, 优点：不需要额外使用struct，每个变体拥有不同的类型以及关联的数量
    #[derive(Debug)]
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    //可以为枚举添加方法
    impl IpAddrKind {
        fn call(&self) {
            println!("值是:{:?}", self)
        }
    }

    #[test]
    fn test_02() {
        let home = IpAddrKind::V4(127, 0, 0, 1);
        let loopback = IpAddrKind::V6(String::from("::1"));
        home.call();//调用枚举的方法
        loopback.call();
        match home {
            IpAddrKind::V4(a, b, c, d) => {println!("{}-{}-{}-{}",a,b,c,d)}
            IpAddrKind::V6(v) => {println!("{}",v)}
        }
    }
}