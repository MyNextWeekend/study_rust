use std::fs::File;
use std::io;
use thiserror::Error;


// 主要用于lib项目中，用于聚合多种错误


#[derive(Error, Debug)]
pub enum DataStoreError {
    // 通过 #[error(transparent)] 让 source 和 Display 直接使用底层的错误
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}

fn read_file() -> Result<(), DataStoreError> {
    File::open("123.123")?;
    Ok(())
}


#[cfg(test)]
mod thiserror_test {
    use super::*;

    #[test]
    fn test01() {
        match read_file() {
            Ok(_) => { println!("666") }
            Err(err) => { println!("错误的类型是:{}", err) } // 错误的类型是:No such file or directory (os error 2)
        }
    }
    #[test]
    fn test02() {
        let error1 = DataStoreError::IoError(io::Error::new(io::ErrorKind::Other, "oh no!"));
        let error2 = DataStoreError::IoError(io::Error::from(io::ErrorKind::TimedOut));
        let error4 = DataStoreError::Redaction("今天".into());
        let error5 = DataStoreError::InvalidHeader { expected: "今天".into(), found: "明天".into() };
        let error6 = DataStoreError::Unknown;
        println!("异常是:{}", error1); // 异常是:oh no!
        println!("异常是:{}", error2); // 异常是:timed out
        println!("异常是:{}", error4); // 异常是:the data for key `今天` is not available
        println!("异常是:{}", error5); // 异常是:invalid header (expected "今天", found "明天")
        println!("异常是:{}", error6); // 异常是:unknown data store error
    }
}