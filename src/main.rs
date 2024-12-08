mod base64;
mod cli;
mod n_triangle;

use clap::Parser;
use cli::{Cli, CliRoute};

fn main() {
    Cli::parse().route();
}
