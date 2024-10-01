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

use clap::Parser;
use crate::test_clap::Args;

fn main() {
    // clap 使用样例
    let args = Args::parse();
    for _ in 0..args.count {
        println!("hello {}", args.name)
    }
}
