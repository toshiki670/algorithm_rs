use clap::Parser;
use cli::{Cli, CliRoute};

mod base64;
mod cli;
mod n_triangle;

fn main() {
    Cli::parse().route();
}
