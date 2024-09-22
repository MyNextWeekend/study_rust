use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    pub(crate) name: String,
    /// Number of times to greet
    #[arg(short, long)]
    pub(crate) count: u8,
}

#[cfg(test)]
mod clap_test {
    #[test]
    fn test01() {
        println!("使用样例放在main.rs")
    }
}