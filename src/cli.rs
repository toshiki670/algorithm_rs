use crate::{
    base64::{encode::EncodeArgs, Base64Args, Base64Subcommand},
    n_triangle::NTriangleArgs,
};
use clap::{Parser, Subcommand};
use enum_dispatch::enum_dispatch;
use env_logger;
use log::Level;
use std::env;

#[enum_dispatch]
pub trait Route {
    fn route(&self);
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Algorithm tool", long_about = None)]
pub struct CliArgs {
    #[arg(short, long, help = "Show logs")]
    pub verbose: bool,

    #[command(subcommand)]
    command: RootSubcommand,
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(Route)]
enum RootSubcommand {
    Base64(Base64Args),
    NTriangle(NTriangleArgs),
}

impl Route for CliArgs {
    fn route(&self) {
        if self.verbose {
            env::set_var("RUST_LOG", Level::Trace.to_string());
        }
        env_logger::init();

        self.command.route();
    }
}
