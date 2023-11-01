use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(required = true)]
    /// Files to be assembled
    pub file: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
