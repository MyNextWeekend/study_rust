use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

static GLOBAL_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    println!("init global data");
    let mut m = HashMap::new();
    m.insert(13, "Spica".to_string());
    m.insert(74, "Hoyten".to_string());
    Mutex::new(m)
});

#[cfg(test)]
mod once_cell_test {

    use super::*;
    #[test]
    fn test01() {
        println!("test func start...");
        {
            let guard = GLOBAL_DATA.lock().unwrap();
            println!("{guard:#?}");
        }
        println!("step one success");
        {
            let guard = GLOBAL_DATA.lock().unwrap();
            println!("{guard:#?}");
        }
        println!("step two success");
        {
            let guard = GLOBAL_DATA.lock().unwrap();
            println!("{guard:#?}");
        }
        println!("step three success");
    }
}
