// 属性标记，该标记会告诉编译器忽略未使用的变量，不要抛出 warning 警告
#![allow(unused_variables)]
// 对全局所有方法生效  ps:也可以在方法上单独加 #[allow(dead_code)]
#![allow(dead_code)]

mod test_sqlx;
mod test_reqwest;
mod test_tokio;

#[tokio::main]
async fn main() {
    println!("Hello from study_async!");
}
