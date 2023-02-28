use clap::{Arg, Command, ArgMatches};
use log::error;

mod calc;

pub fn cli() -> Command {
    Command::new("n_triangle")
        .version("1.2.0")
        .author("Toshiki <bushy.trivial.0o@icloud.com>")
        .about("n_triangle calcurater")
        .arg(
            Arg::new("height")
                .value_name("HEIGHT")
                .help("explain what is being done")
                .required(true)
        )
}

pub fn exec(matches: &ArgMatches) {
    let height = matches.get_one::<String>("height").expect("required");

    match height.parse() {
        Ok(h) => println!("The answer with a height of {} is {}.", h, calc::call(h)),
        Err(e) => {
            error!("ParseIntError: {}.", e);
            error!("Please specfy integer value: {}.", &height);
        },
    }
}
