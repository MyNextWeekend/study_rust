#[cfg(test)]
mod time_test {
    use std::thread;
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    #[test]
    fn test01() {
        let start = Instant::now();
        println!("{:?}", start);
        thread::sleep(Duration::from_secs(3));
        println!("耗时是:{:?}", start.elapsed());
    }

    #[test]
    fn test02() {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        println!("时间戳-秒:{}", start.as_secs());
        println!("时间戳-毫秒:{}", start.as_millis());
        println!("时间戳-微秒:{}", start.as_micros());
        println!("时间戳-纳秒:{}", start.as_nanos());
    }
}