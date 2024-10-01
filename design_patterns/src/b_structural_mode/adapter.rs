// 目标接口
trait Printer {
    fn print(&self);
}

// 旧的打印机类
struct OldPrinter {
    text: String,
}

impl OldPrinter {
    fn new(text: &str) -> Self {
        OldPrinter { text: text.to_string() }
    }

    fn old_print(&self) {
        println!("Old Printer: {}", self.text);
    }
}

// 适配器
struct PrinterAdapter<'a> {
    old_printer: &'a OldPrinter,
}

impl<'a> Printer for PrinterAdapter<'a> {
    fn print(&self) {
        self.old_printer.old_print();
    }
}


#[cfg(test)]
mod adapter_test {
    use super::*;

    #[test]
    fn test_01() {
        // 客户端
        let old_printer = OldPrinter::new("Hello, Adapter Pattern!");
        let adapter = PrinterAdapter { old_printer: &old_printer };

        // 使用适配器
        adapter.print();
    }
}


