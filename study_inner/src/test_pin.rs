use std::collections::HashMap;
use std::marker::PhantomPinned;
use std::pin::Pin;

// 包装 引擎
struct Engine {
    map: HashMap<String, String>,
    executor: Executor,
    _marker: PhantomPinned, // 一个标记字段: !unpin ,导致整个结构体是 !unpin
}
impl Engine {
    // pin 一个 unpin 的结构体是无效的，需要包裹 !unpin 才有意义
    fn new(map: HashMap<String, String>) -> Pin<Box<Self>> {
        // 1️⃣ 先 Pin 住整个对象
        let mut engine = Box::pin(Self {
            map,
            executor: Executor {
                map: std::ptr::null(),
            },
            _marker: PhantomPinned,
        });

        // 2️⃣ 通过 as_ref().get_ref() 访问 map 字段
        let map_ptr: *const HashMap<String, String> = &engine.map;

        // 3️⃣ 在 unsafe 块中修改内部 executor
        unsafe {
            let mut_ref = engine.as_mut().get_unchecked_mut();
            mut_ref.executor.map = map_ptr;
        }

        engine
    }

    // 包装执行器前后做一些操作
    fn do_stuff(&self, key: &str) -> String {
        println!("Engine do stuff:{}，len:{}", key, self.map.len());
        if let Some(result) = self.executor.do_stuff(key) {
            result
        } else {
            String::from("HELLO")
        }
    }
}

// 真正的执行者
struct Executor {
    // 使用 原始指针
    map: *const HashMap<String, String>,
}

impl Executor {
    fn do_stuff(&self, key: &str) -> Option<String> {
        unsafe { &*self.map }.get(key).map(|e| String::from(e))
    }
}

#[cfg(test)]
mod pin_test {
    use super::*;
    #[test]
    fn test_pin() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), "value".to_string());
        map.insert("key2".to_string(), "value2".to_string());
        let engine = Engine::new(map);
        println!("Engine get result: {:#?}", engine.do_stuff("key"));
    }
}
