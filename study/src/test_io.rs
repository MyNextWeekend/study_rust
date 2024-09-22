use std::fs::{File, OpenOptions, remove_file};
use std::io;
use std::io::{BufRead, BufReader, Read, Write};


/// 创建并写入到文件
fn create_write_file(file_path: &str, content: &str) -> Result<usize, io::Error> {
    let mut file = File::create(&file_path)?;
    let num = file.write(content.as_bytes())?;
    println!("文件路径:{file_path} 成功写入{num}字节", );
    Ok(num)
}

/// 追加模式往文件中写入
fn append_write_file(file_path: &str, content: &str) -> Result<usize, io::Error> {
    let mut file = OpenOptions::new().append(true).open(&file_path)?;
    let num = file.write(content.as_bytes())?;
    println!("文件路径:{file_path} 成功追加写入{num}字节", );
    Ok(num)
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(&file_path)?;
    let mut result = String::new();
    file.read_to_string(&mut result)?;
    println!("整个读取文件内容是: {}", result);
    Ok(result)
}

/// 按行遍历文件内容
fn read_file_iter(file_path: &str) -> Result<io::Lines<BufReader<File>>, io::Error> {
    let file = File::open(&file_path)?;
    Ok(BufReader::new(file).lines())
}

/// 删除文件
fn del_file(file_path: &str) -> Result<(), io::Error> {
    let _ = remove_file(file_path)?;
    println!("删除文件成功");
    Ok(())
}

#[cfg(test)]
mod io_test {
    use super::*;

    #[test]
    fn test_01() {
        let file_path = "./test.txt";
        println!("========= 创建并写入 ===========");
        create_write_file(&file_path, "这个是需要写入的内容\n").unwrap();


        println!("========= 追加模式写入 ===========");
        append_write_file(&file_path, "这个是追加的内容\n").unwrap();


        println!("========= 整个读取 ===========");
        read_file(&file_path).unwrap();

        println!("========= 按行读取 ===========");
        for line in read_file_iter(&file_path).unwrap() {
            if let Ok(txt) = line {
                println!("{txt}");
            }
        }

        println!("========= 删除文件 ===========");
        del_file(&file_path).unwrap();
    }
}
