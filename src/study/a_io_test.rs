use std::{fs::File, io::{BufRead, BufReader, Read}};

#[test]
fn test_01() {
     // 整个读取
    let mut file = File::open("./Cargo.toml").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    print!("文件内容是: {}", s);

    println!("====================");
    // 按行读取
    let file = File::open("./Cargo.toml").unwrap();
    for line in BufReader::new(file).lines(){
        if let Ok(txt) = line {
            println!("按照行读取: {}", txt);
        }   
    }
}
