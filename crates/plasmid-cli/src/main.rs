#![forbid(unsafe_code)]

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);

    // 1. Plan
    // 2. Apply
    //  a. Plan
    //  b. Execute plan
}
