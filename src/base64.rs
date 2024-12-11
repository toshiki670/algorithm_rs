pub mod encode;

use crate::cli::Route;
use encode::EncodeArgs;

#[derive(clap::Args, Debug)]
#[command(version = "0.1.0")]
#[command(author)]
#[command(about = "base64 algorithm", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
#[enum_delegate::implement(Route)]
pub enum Subcommand {
    Encode(EncodeArgs),
}

impl Route for Args {
    fn route(&self) {
        self.command.route();
    }
}
