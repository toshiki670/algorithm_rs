use std::env;

use clap::{App, SubCommand};
use env_logger;
use log::{error, Level};

mod n_triangle;

fn main() {
    let cli = clap::load_yaml!("cli.yml");
    let n_triangle_cli = clap::load_yaml!("n_triangle/cli.yml");

    let matches = App::from_yaml(cli)
        .subcommand(SubCommand::from_yaml(n_triangle_cli))
        .get_matches();

    if matches.is_present("verbose") {
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
