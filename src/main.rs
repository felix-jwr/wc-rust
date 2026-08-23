use clap::Parser;
use wc_rust::{InputArgs, resolve_defaults};

fn main() {
    let mut args = InputArgs::parse();
    resolve_defaults(&mut args);
    println!("Hello, world!");
}
