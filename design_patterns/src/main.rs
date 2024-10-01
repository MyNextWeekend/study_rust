// 属性标记，该标记会告诉编译器忽略未使用的变量，不要抛出 warning 警告
#![allow(unused_variables)]
// 对全局所有方法生效  ps:也可以在方法上单独加 #[allow(dead_code)]
#![allow(dead_code)]


/// 创建模式
mod a_creation_mode;

/// 结构模式
mod b_structural_mode;

/// 行为模式
mod c_behavioral_pattern;

fn main() {
    println!("Hello from design_patterns!");
}
