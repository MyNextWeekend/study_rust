use clap::Parser;

/// bilibili下载器
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Bilibili {
    /// The input file to use
    #[arg(short, long)]
    input: String,

    /// The output file to use
    #[arg(short, long)]
    output: String,

    /// The number of threads to use
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}
impl Bilibili {
    fn download(&self) {
        // 下载逻辑
        println!(
            "Bilibili Downloading from {} to {} using {} threads",
            self.input, self.output, self.threads
        );
    }
}

/// 抖音下载器
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Douyin {
    /// 下载链接
    #[arg(short, long)]
    input: String,

    /// 输入位置
    #[arg(short, long)]
    output: String,

    /// 线程数量
    #[arg(short, long, default_value_t = 1)]
    threads: usize,
}

impl Douyin {
    fn download(&self) {
        // 下载逻辑
        println!(
            "Douyin Downloading from {} to {} using {} threads",
            self.input, self.output, self.threads
        );
    }
}

/// 易用下载器
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub enum MyEnum {
    Bilibili(Bilibili),
    Douyin(Douyin),
}

fn main() {
    let arg = MyEnum::parse();
    println!("{:?}", arg);
    match arg {
        MyEnum::Bilibili(bilibili) => {
            bilibili.download();
        }
        MyEnum::Douyin(douyin) => {
            douyin.download();
        }
    }
}
