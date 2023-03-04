use std::env;

use clap::{Arg, Command, ArgAction};
use env_logger;
use log::Level;

mod base64;
mod n_triangle;

fn cli() -> Command {
    Command::new("Algorithm")
        .version("1.1.0")
        .author("Toshiki <bushy.trivial.0o@icloud.com>")
        .about("Algorithm tool")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("explain what is being done")
                .action(ArgAction::SetTrue)
        )
        .subcommand(base64::cli())
        .subcommand(n_triangle::cli())
}

fn main() {
    let matches = cli().get_matches();

    if matches.get_flag("verbose") {
        env::set_var("RUST_LOG", Level::Trace.to_string());
    }
    env_logger::init();

    match matches.subcommand() {
        Some(("base64", sub_matches)) => {
            base64::exec(sub_matches);
        }
        Some(("n_triangle", sub_matches)) => {
            n_triangle::exec(sub_matches);
        }
        _ => unreachable!()
    }

    println!("\nGood bye!");
}
