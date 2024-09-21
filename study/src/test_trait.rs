// 特性（trait）概念接近于 Java 中的接口（Interface），但两者不完全相同。
// 特性与接口相同的地方在于它们都是一种行为规范，可以用于标识哪些类有哪些方法。
trait Biology {
    fn eat(&self);
    fn run(&self) {
        println!("在运动。。");
    }
}

struct Person {
    name: String,
    age: u8,
}

impl Biology for Person {
    fn eat(&self) {
        println!("{}正在吃饭", self.name)
    }
}

#[cfg(test)]
mod trait_test {
    use crate::test_trait::{Biology, Person};

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
}
