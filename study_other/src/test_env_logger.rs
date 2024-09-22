use log::{Level, LevelFilter};
use log::{debug, error, info, trace};
use std::io::Write;


fn init_logger() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "debug");

    // 设置打印的格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            // 设置不同等级日志的颜色
            let lever_color = match record.level() {
                Level::Error => env_logger::fmt::Color::Red,
                Level::Warn => env_logger::fmt::Color::Yellow,
                Level::Info => env_logger::fmt::Color::Green,
                Level::Debug => env_logger::fmt::Color::Cyan,
                Level::Trace => env_logger::fmt::Color::Cyan
            };

            // 样式一
            let mut lever_style = buf.style();
            lever_style.set_color(lever_color).set_bold(true);

            // 样式二
            let mut style = buf.style();
            style.set_color(env_logger::fmt::Color::White).set_dimmed(true);

            writeln!(
                buf,
                "{} {} [{}:{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                lever_style.value(record.level()), // 样式使用 或者不使用record.level()
                style.value(record.module_path().unwrap_or("<unnamed>")),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    info!("env_logger init success")
}

fn do_001() {
    init_logger();
    trace!("trace");
    debug!("this is a debug {}", "message");
    info!("info");
    error!("this is printed by default");
}


#[cfg(test)]
mod env_logger_test {
    use super::*;

    #[test]
    fn test01() {
        do_001()
    }
}
