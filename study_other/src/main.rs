#![allow(dead_code)]
#![allow(unused_variables)]

mod test_anyhow;
mod test_calamine;
mod test_chrono;
mod test_clap;
mod test_env_logger;
mod test_reqwest;
mod test_serde;
mod test_sqlx;
mod test_thiserror;

use clap::Parser;
use crate::test_clap::Args;

fn main() {
    // 使用样例 - clap
    let args = Args::parse();
    for _ in 0..args.count {
        println!("hello {}", args.name)
    }
}
