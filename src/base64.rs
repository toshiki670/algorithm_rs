pub mod encode;

use crate::cli::CliRoute;
use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;

#[derive(Args, Debug)]
#[command(version = "0.1.0")]
#[command(author)]
#[command(about = "base64 algorithm", long_about = None)]
pub struct Base64 {
    #[command(subcommand)]
    command: Base64Commands,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(CliRoute)]
pub enum Base64Commands {
    Encode(encode::Encode),
}

impl CliRoute for Base64 {
    fn route(&self) {
        self.command.route();
    }
}
