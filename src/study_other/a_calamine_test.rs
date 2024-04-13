#[cfg(test)]
mod calamine_test {
    use calamine::{open_workbook, Reader, Xlsx};

    #[test]
    fn test01() {
        let workbook: Result<Xlsx<_>, _> = open_workbook("file.xlsx");
        if let Ok(mut excel) = workbook {
            if let Ok(data) = excel.worksheet_range("Sheet1") {
                for row in data.rows() {
                    println!("row={:?}, row[0]={:?}", row, row[0]);
                }
            }
        }
    }
}