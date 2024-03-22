// 泛型学习

// 泛型方法
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

// 泛型结构体
struct Proper<T> {
    name: String,
    age: i32,
    hobby: T,
}
impl<T> Proper<T> {
    fn new(hobby: T) -> Self {
        Proper {
            name: String::from("value"),
            age: 18,
            hobby: hobby,
        }
    }
}

#[test]
fn test_01() {
    let a = [2, 4, 6, 3, 1];  
    println!("max = {}", max(&a));

    let user = Proper::new(vec!["唱".to_string(),"跳".to_string(),"rap".to_string()]);
    println!("{} {} {}",user.age,user.name,user.hobby[0])
}
