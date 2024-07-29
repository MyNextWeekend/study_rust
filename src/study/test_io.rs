#[cfg(test)]
mod io_test {
    use std::fs::{File, OpenOptions};
    use std::io::{BufRead, BufReader, Read, Write, BufWriter};

    #[test]
    fn test_01() {
        let file_path = "./test.txt";
        println!("========= 创建并写入 ===========");
        if let Ok(mut file) = File::create(&file_path) {
            &file.write("写入内容-->abcef123456...\n".as_bytes()).unwrap();
            println!("文件:{:?}", file);
        } else {
            println!("文件创建失败");
        }


        println!("========= 追加模式写入 ===========");
        let mut file = OpenOptions::new().append(true).open(&file_path).unwrap();
        &file.write("追加内容-->abcef123456...\n".as_bytes()).unwrap();
        println!("文件:{:?}", file);

        println!("========= 整个读取 ===========");
        let mut file = File::open(&file_path).unwrap();
        let mut result = String::new();
        file.read_to_string(&mut result).unwrap();
        println!("整个读取文件内容是: {}", result);

        println!("========= 按行读取 ===========");
        let file = File::open(&file_path).unwrap();
        for line in BufReader::new(file).lines() {
            if let Ok(txt) = line {
                println!("按照行读取: {}", txt);
            }
        }
    }
}
