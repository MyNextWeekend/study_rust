// 方式一：结构体中直接写字段名称
enum IpAddr {
    V4,
    V6,
}

// 方式二：将数据附加到枚举的变体中（利用struct）
struct IpAddrWithStr {
    kind: IpAddr,
    address: String,
}

// 方式三：将数据附加到枚举的变体中, 优点：不需要额外使用struct，每个变体拥有不同的类型以及关联的数量
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

//可以为枚举添加方法
impl IpAddrKind {
    fn call(&self) {
        println!("枚举方法被调用。。")
    }
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
}

#[test]
fn test_02() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    home.call() //调用枚举的方法
}
