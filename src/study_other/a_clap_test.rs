use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,
    /// Number of times to greet
    #[arg(short, long)]
    count: u8,
}

#[cfg(test)]
mod clap_test {
    use clap::Parser;
    use crate::study_other::a_clap_test::Args;

    #[test]
    fn test01() {
        let args = Args::parse();
        for _ in 0..args.count {
            println!("hello {}",args.name)
        }
    }
}