use clap::Parser;

mod args;
mod stage1;

#[derive(Parser)]
#[clap(name = "bunker", version = "1.0", author = "matrix")]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    println!("File: {:?}", args.file);
}
