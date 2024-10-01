// 抽象类，定义模板方法
trait DataProcessor {
    // 模板方法
    fn process(&self) {
        self.read_data();
        self.process_data();
        self.write_data();
    }

    // 抽象方法，由子类实现
    fn read_data(&self);
    fn process_data(&self);
    fn write_data(&self);
}

// 子类实现具体的处理逻辑
struct CSVProcessor;

impl DataProcessor for CSVProcessor {
    fn read_data(&self) {
        println!("Reading data from CSV file.");
    }

    fn process_data(&self) {
        println!("Processing CSV data.");
    }

    fn write_data(&self) {
        println!("Writing data to CSV file.");
    }
}

// 另一个子类
struct JSONProcessor;

impl DataProcessor for JSONProcessor {
    fn read_data(&self) {
        println!("Reading data from JSON file.");
    }

    fn process_data(&self) {
        println!("Processing JSON data.");
    }

    fn write_data(&self) {
        println!("Writing data to JSON file.");
    }
}


#[cfg(test)]
mod template_test {
    use super::*;

    #[test]
    fn test_01() {
        let csv_processor = CSVProcessor;
        let json_processor = JSONProcessor;

        println!("Processing CSV:");
        csv_processor.process();

        println!("\nProcessing JSON:");
        json_processor.process();
    }
}


