// 主要用于lib项目中，用于聚合多种错误

use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
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

#[cfg(test)]
mod thiserror_test {
    use std::io;
    use std::io::ErrorKind;
    use crate::study_other::test_thiserror::DataStoreError;

    #[test]
    fn test01() {
        let error1 = DataStoreError::Disconnect(io::Error::new(ErrorKind::Other, "oh no!"));
        let error2 = DataStoreError::Redaction("今天".into());
        let error3 = DataStoreError::InvalidHeader { expected: "今天".into(), found: "明天".into() };
        let error4 = DataStoreError::Unknown;
        println!("异常是:{}", error1);
        println!("异常是:{}", error2);
        println!("异常是:{}", error3);
        println!("异常是:{}", error4);
    }
}