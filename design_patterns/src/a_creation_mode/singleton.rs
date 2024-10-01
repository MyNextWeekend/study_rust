use once_cell::sync::OnceCell;

struct Singleton {
    value: i32,
}

impl Singleton {
    fn new() -> Self {
        Singleton { value: 42 }
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

static INSTANCE: OnceCell<Singleton> = OnceCell::new();

fn get_instance() -> &'static Singleton {
    INSTANCE.get_or_init(Singleton::new)
}


#[cfg(test)]
mod singleton_test {
    use super::*;

    #[test]
    fn test_01() {
        let instance = get_instance();
        println!("Singleton value: {}", instance.get_value());
    }
}

