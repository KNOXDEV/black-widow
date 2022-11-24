use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    target: String,
    #[arg(short, long, default_value_t = 5)]
    threads: u8,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("hello {}", args.target);
}
