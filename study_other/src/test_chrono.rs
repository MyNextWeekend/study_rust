use chrono::{Local, Utc, NaiveDateTime, NaiveDate, Months, Days, Datelike};


fn parse_time(time_str: &str) -> Result<NaiveDateTime, String> {
    if let Ok(res) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        return Ok(res);
    };
    if let Ok(res) = NaiveDateTime::parse_from_str(time_str, "%Y/%m/%d %H:%M:%S") {
        return Ok(res);
    };
    if let Ok(res) = NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d") {
        return Ok(res);
    };
    if let Ok(res) = NaiveDateTime::parse_from_str(time_str, "%Y/%m/%d") {
        return Ok(res);
    };
    Err(String::from("格式化异常"))
}

fn to_midnight(datetime: NaiveDateTime) -> Result<NaiveDateTime, String> {
    let date = NaiveDate::from_ymd_opt(datetime.year(), datetime.month(), datetime.day()).ok_or("Invalid date")?;
    let time = date.and_hms_opt(0, 0, 0).unwrap();
    Ok(time)
}

fn add_days(datetime: NaiveDateTime, days: i32) -> Option<NaiveDateTime> {
    if days == 0 {
        return Some(datetime);
    } else if days > 0 {
        return datetime.checked_add_days(Days::new(days as u64));
    } else {
        datetime.checked_sub_days(Days::new(-days as u64))
    }
}

fn add_months(datetime: NaiveDateTime, months: i32) -> Option<NaiveDateTime> {
    if months == 0 {
        return Some(datetime);
    } else if months > 0 {
        return datetime.checked_add_months(Months::new(months as u32));
    } else {
        datetime.checked_sub_months(Months::new(-months as u32))
    }
}


#[cfg(test)]
mod chrono_test {
    use super::*;

    #[test]
    fn test01() {
        let now1 = Utc::now(); // 获取 世界统一时间的现在时间
        let now2 = Local::now(); // 获取当地时间的现在时间
        println!("格式化为字符串:{}", now1.format("%Y-%m-%d %H:%M:%S"));
        println!("格式化为字符串:{}", now2.format("%Y-%m-%d"));
    }

    #[test]
    fn test02() {
        let now = parse_time("2015-09-05 23:56:04").unwrap();
        println!("解析字符串，得到的日期时间是：{:?}", now);
        println!("转换毫秒数是：{:?}", now.and_utc().timestamp_millis());

        let now = to_midnight(now).unwrap();
        println!("凌晨时间，得到的日期是：{:?}", now);
        let now = add_months(now, -3).unwrap();
        println!("减少3个月时间，得到的日期是：{:?}", now);
        let now = add_days(now, 3).unwrap();
        println!("增加3天的时间，得到的日期是：{:?}", now);
    }
}