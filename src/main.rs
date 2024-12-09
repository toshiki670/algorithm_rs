mod base64;
mod cli;
mod n_triangle;
mod system;

use clap::Parser;
use cli::{CliArgs, Route};

fn main() {
    CliArgs::parse().route();
}
