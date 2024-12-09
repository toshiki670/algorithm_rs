mod base64;
mod cli;
mod n_triangle;

use clap::Parser;
use cli::{Args, Route};

fn main() {
    Args::parse().route();
}
