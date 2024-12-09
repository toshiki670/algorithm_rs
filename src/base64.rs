pub mod encode;

use crate::cli::Route;
use clap::{Args, Subcommand};
use encode::EncodeArgs;
use enum_dispatch::enum_dispatch;

#[derive(Args, Debug)]
#[command(version = "0.1.0")]
#[command(author)]
#[command(about = "base64 algorithm", long_about = None)]
pub struct Base64Args {
    #[command(subcommand)]
    command: Base64Subcommand,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(Route)]
pub enum Base64Subcommand {
    Encode(EncodeArgs),
}

impl Route for Base64Args {
    fn route(&self) {
        self.command.route();
    }
}
