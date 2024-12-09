use crate::base64::encode;
use crate::base64::Base64Commands;
use crate::{base64, n_triangle};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use env_logger;
use log::Level;
use std::env;

#[enum_dispatch]
pub trait CliRoute {
    fn route(&self);
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Algorithm tool", long_about = None)]
pub struct Args {
    #[arg(short, long, help = "Show logs")]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(CliRoute)]
enum Commands {
    Base64(base64::Base64),
    NTriangle(n_triangle::NTriangle),
}

impl CliRoute for Cli {
    fn route(&self) {
        if self.verbose {
            env::set_var("RUST_LOG", Level::Trace.to_string());
        }
        env_logger::init();

        self.command.route();
    }
}
