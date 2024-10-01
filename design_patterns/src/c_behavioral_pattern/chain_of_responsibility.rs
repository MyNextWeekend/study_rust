// 处理者接口
trait Handler {
    fn set_next(&mut self, handler: Box<dyn Handler>);
    fn handle(&self, request: &str) -> Option<String>;
}

// 具体处理者：级别1
struct ConcreteHandlerA {
    next_handler: Option<Box<dyn Handler>>,
}

impl ConcreteHandlerA {
    fn new() -> Self {
        ConcreteHandlerA { next_handler: None }
    }
}

impl Handler for ConcreteHandlerA {
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.next_handler = Some(handler);
    }

    fn handle(&self, request: &str) -> Option<String> {
        if request == "A" {
            Some(String::from("Handled by Handler A"))
        } else {
            if let Some(ref next) = self.next_handler {
                next.handle(request)
            } else {
                None
            }
        }
    }
}

// 具体处理者：级别2
struct ConcreteHandlerB {
    next_handler: Option<Box<dyn Handler>>,
}

impl ConcreteHandlerB {
    fn new() -> Self {
        ConcreteHandlerB { next_handler: None }
    }
}

impl Handler for ConcreteHandlerB {
    fn set_next(&mut self, handler: Box<dyn Handler>) {
        self.next_handler = Some(handler);
    }

    fn handle(&self, request: &str) -> Option<String> {
        if request == "B" {
            Some(String::from("Handled by Handler B"))
        } else {
            if let Some(ref next) = self.next_handler {
                next.handle(request)
            } else {
                None
            }
        }
    }
}

// 具体处理者：级别3
struct ConcreteHandlerC;

impl Handler for ConcreteHandlerC {
    fn set_next(&mut self, _handler: Box<dyn Handler>) {
        // 不需要设置下一处理者
    }

    fn handle(&self, request: &str) -> Option<String> {
        if request == "C" {
            Some(String::from("Handled by Handler C"))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod chain_of_responsibility_test {
    use super::*;


    #[test]
    fn test_01() {
        let mut handler_a = ConcreteHandlerA::new();
        let handler_b = ConcreteHandlerB::new();
        let handler_c = ConcreteHandlerC;

        // 设置责任链
        handler_a.set_next(Box::new(handler_b));
        handler_a.next_handler.as_mut().unwrap().set_next(Box::new(handler_c));

        // 测试请求
        let requests = vec!["A", "B", "C", "D"];

        for request in requests {
            match handler_a.handle(request) {
                Some(response) => println!("Request: {}, Response: {}", request, response),
                None => println!("Request: {}, Response: Not Handled", request),
            }
        }
    }
}

