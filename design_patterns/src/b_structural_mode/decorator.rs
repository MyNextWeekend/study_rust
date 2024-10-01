// 基础接口
trait Coffee {
    fn cost(&self) -> f64; // 获取咖啡的价格
    fn description(&self) -> String; // 获取咖啡的描述，返回 String
}

// 基础咖啡类
struct SimpleCoffee;

impl Coffee for SimpleCoffee {
    fn cost(&self) -> f64 {
        2.0 // 基础咖啡价格
    }

    fn description(&self) -> String {
        "Simple Coffee".to_string() // 返回 String
    }
}

// 装饰器结构体
struct MilkDecorator<'a> {
    coffee: &'a dyn Coffee,
}

impl<'a> Coffee for MilkDecorator<'a> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.5 // 加牛奶增加价格
    }

    fn description(&self) -> String {
        format!("{} + Milk", self.coffee.description()) // 返回 String
    }
}

// 另一种装饰器
struct SugarDecorator<'a> {
    coffee: &'a dyn Coffee,
}

impl<'a> Coffee for SugarDecorator<'a> {
    fn cost(&self) -> f64 {
        self.coffee.cost() + 0.2 // 加糖增加价格
    }

    fn description(&self) -> String {
        format!("{} + Sugar", self.coffee.description()) // 返回 String
    }
}


#[cfg(test)]
mod decorator_test {
    use super::*;


    #[test]
    fn test_01() {
        let simple_coffee = SimpleCoffee;

        // 使用装饰器
        let milk_coffee = MilkDecorator { coffee: &simple_coffee };
        let sugar_milk_coffee = SugarDecorator { coffee: &milk_coffee };

        println!("Description: {}", sugar_milk_coffee.description());
        println!("Cost: ${:.2}", sugar_milk_coffee.cost());
    }
}


