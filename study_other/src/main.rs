#![allow(dead_code)]
#![allow(unused_variables)]

mod test_anyhow;
mod test_calamine;
mod test_chrono;
mod test_clap;
mod test_env_logger;
mod test_serde;
mod test_thiserror;
mod test_dotenv;
mod test_indicatif;


fn main() {
    // 进度条
    // test_indicatif::progress();

    // clap 使用样例
    use clap::Parser;

    let args = test_clap::Args::parse();
    for _ in 0..args.count {
        println!("hello {}", args.name)
    }
}
