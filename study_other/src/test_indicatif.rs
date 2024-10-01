/// 进度条

use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time};

pub(crate) fn progress() {
    // 创建一个进度条，设置最大值
    let bar = ProgressBar::new(100);

    // 设置进度条的样式
    bar.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40}] {percent:>3}% | ETA: {eta}")
        .unwrap().progress_chars("##-"));

    // 模拟一个长时间运行的任务
    for i in 0..100 {
        // 更新进度条
        bar.set_message(format!("Processing {}", i + 1));
        bar.inc(1); // 增加进度
        thread::sleep(time::Duration::from_millis(50)); // 模拟工作
    }

    bar.finish(); // 完成进度条
    println!("Done!");
}


#[cfg(test)]
mod indicatif_test {
    use super::*;

    #[test]
    fn test_01() {
        progress();
    }
}