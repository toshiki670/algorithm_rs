use crate::{base64, n_triangle, system};
use clap::Parser;
use env_logger;
use log::Level;
use std::env;

#[enum_delegate::register]
pub trait Route {
    fn route(&self);
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Algorithm tool", long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Show logs")]
    pub verbose: bool,

    #[command(subcommand)]
    command: Subcommand,
}

#[derive(clap::Subcommand, Debug)]
#[enum_delegate::implement(Route)]
enum Subcommand {
    Base64(base64::Args),
    NTriangle(n_triangle::Args),
    System(system::Args),
}

impl Route for Args {
    fn route(&self) {
        if self.verbose {
            env::set_var("RUST_LOG", Level::Trace.to_string());
        }
        env_logger::init();

        self.command.route();
    }
}
