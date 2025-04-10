use std::path::{Path, PathBuf};

fn do_something() {
    let root = Path::new(".env"); // Path不允许修改
    println!("文件的路径是: {}", root.display());
    println!("{:?}", root.join("12")); // 这里类型做了转换


    let mut path_buf = PathBuf::new();
    let mut path_buf = path_buf.join("hello");
    path_buf.push("world");
    println!("{}", path_buf.display());
    if path_buf.is_file() {
        println!("文件存在")
    }
}

#[cfg(test)]
mod func_test {
    use super::*;

    #[test]
    fn test_01() {
        do_something()
    }
}