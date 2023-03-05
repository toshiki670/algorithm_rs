use std::env;

use clap::{Parser, Subcommand};
use env_logger;
use log::Level;

mod base64;
mod n_triangle;


#[derive(Parser, Debug)]
#[command(author, version, about = "Algorithm tool", long_about = None)]
pub struct Cli {
    #[arg(short, long, help = "Show logs")]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Base64(base64::Base64),
    NTriangle(n_triangle::NTriangle),
}

fn main() {
    // let matches = cli().get_matches();
    let cli = Cli::parse();

    if cli.verbose {
        env::set_var("RUST_LOG", Level::Trace.to_string());
    }
    env_logger::init();

    match &cli.command {
        Commands::Base64(cli) => {
            base64::exec(cli);
        },
        Commands::NTriangle(cli) => {
            n_triangle::exec(cli);
        },
    }

    println!("\nGood bye!");
}
