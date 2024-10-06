use std::num::ParseIntError;
use std::str::FromStr;

// 特性（trait）概念接近于 Java 中的接口（Interface），但两者不完全相同。
// 特性与接口相同的地方在于它们都是一种行为规范，可以用于标识哪些类有哪些方法。
trait Biology {
    fn eat(&self);
    fn run(&self) {
        println!("在运动。。");
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Biology for Person {
    fn eat(&self) {
        println!("{}正在吃饭", self.name)
    }
}

// 定义一个解析错误类型
#[derive(Debug)]
enum ParsePersonError {
    InvalidFormat,
    InvalidAge(ParseIntError),
}

// 实现 FromStr trait 自动获取 parse 能力
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 假设字符串格式为 "name,age"
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(ParsePersonError::InvalidFormat);
        }

        let name = parts[0].to_string();
        let age = parts[1].parse::<u8>().map_err(ParsePersonError::InvalidAge)?; // 转换并处理错误

        Ok(Person { name, age })
    }
}

#[cfg(test)]
mod trait_test {
    use super::*;

    #[test]
    fn test01() {
        let cali = Person {
            name: String::from("Cali"),
            age: 24,
        };
        cali.eat();
        cali.run();
        println!("{}的年龄是{}", cali.name, cali.age)
    }

    #[test]
    fn test02() {
        let input = "Alice,30";

        // match Person::from_str(input) {
        //     Ok(person) => println!("Parsed Person: {:?}", person),
        //     Err(e) => println!("Failed to parse Person: {:?}", e),
        // }

        // match input.parse::<Person>() {
        //     Ok(person) => println!("Parsed Person: {:?}", person),
        //     Err(e) => println!("Failed to parse Person: {:?}", e),
        // }

        let p: Person = input.parse().unwrap();
        println!("Person:{:?}", p)
    }
}
