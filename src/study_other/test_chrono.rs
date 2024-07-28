#[cfg(test)]
mod chrono_test {
    use std::fmt::{Debug, Display};
    use chrono::{Local, TimeZone, Utc, NaiveDateTime, NaiveDate, Months, Days};

    #[test]
    fn test01() {
        let now1 = Utc::now(); // 获取 世界统一时间的现在时间
        let now2 = Local::now(); // 获取当地时间的现在时间
        println!("格式化为字符串:{}", now1.format("%Y-%m-%d %H:%M:%S"));
        println!("格式化为字符串:{}", now2.format("%Y-%m-%d"));
    }

    #[test]
    fn test02() {
        let now = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S").unwrap();
        println!("解析字符串，得到的日期时间是：{:?}", now);
        println!("转换毫秒数是：{:?}", now.and_utc().timestamp_millis());

        let now = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d").unwrap();
        println!("解析字符串，得到的日期是：{:?}", now);
        let now = now.checked_add_months(Months::new(3)).unwrap();
        println!("添加三个月时间，得到的日期是：{:?}", now);
        let now = now.checked_sub_days(Days::new(2)).unwrap();
        println!("减少两天的时间，得到的日期是：{:?}", now);
    }
}