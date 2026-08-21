use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Felix", version = "0.1.0")]
#[command(about = "Implementation of the word counter utility 'wc' in Rust")]
struct InputArgs {
    #[command(flatten)]
    output_mode: OutputMode,

    /// Write the number of words in each input file
    #[arg(short, long)]
    w: bool,

    /// Write the number of <newline> characters in each input file
    #[arg(short, long)]
    l: bool,

    /// A pathname of an input file. If no file operands are specified, use stdin
    #[arg()]
    file: Vec<PathBuf>,
}

#[derive(Args, Debug)]
#[group(required = false, multiple = false)]
struct OutputMode {
    /// Write the number of bytes in each input file
    #[arg(short, long)]
    c: bool,

    /// Write the number of characters in each input file
    #[arg(short, long)]
    m: bool,
}

fn main() {
    let mut args = InputArgs::parse();

    if !args.output_mode.c && !args.output_mode.m && !args.l && !args.w {
        args.l = true;
        args.w = true;
        args.output_mode.c = true;
    }

    println!("Hello, world!");
}
