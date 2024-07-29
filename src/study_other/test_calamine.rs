use calamine::{open_workbook, Error, Xlsx, Reader, RangeDeserializerBuilder};


pub fn read_xlsx(file_path: &str) -> Result<(), Error> {
    let mut work_book: Xlsx<_> = open_workbook(file_path)?;
    let sheet = work_book.worksheet_range("Sheet1")
        .map_err(|_| calamine::Error::Msg("Cannot find Sheet1"))?;
    let iter_records =
        RangeDeserializerBuilder::with_headers(&["name", "age"]).from_range(&sheet)?;

    for result in iter_records {
        let (label, value): (String, f64) = result?;
        println!("name={:?}, age={:?}", label, value);
    }
    Ok(())
}


#[cfg(test)]
mod calamine_test {
    use crate::study_other::test_calamine::read_xlsx;

    #[test]
    fn test01() {
        read_xlsx("/Users/weekend/Desktop/test.xlsx").expect("TODO: panic message");
    }
}