use std::env;

use clap::{Arg, Command, ArgAction};
use env_logger;
use log::{error, Level};

mod n_triangle;

fn cli() -> Command {
    Command::new("Algorithm")
        .version("1.0.1")
        .author("Toshiki <bushy.trivial.0o@icloud.com>")
        .about("Algorithm tool")
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("explain what is being done")
                .action(ArgAction::SetTrue)
        )
        .subcommand(n_triangle::cli())
}

fn main() {
    let matches = cli().get_matches();

    if matches.get_flag("verbose") {
        env::set_var("RUST_LOG", Level::Trace.to_string());
    }
    env_logger::init();

    if let Some(m) = matches.subcommand_matches("n_triangle") {
        n_triangle::exec(m);
    } else {
        error!("Do nothing...");
    }

    println!("\nGood bye!");
}
