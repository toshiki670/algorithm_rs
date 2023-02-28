use clap::{Arg, Command, ArgMatches};

pub fn cli() -> Command {
    Command::new("enc")
        .about("Encoder")
        .arg(
            Arg::new("string")
                .value_name("TEXT")
                .help("Encode text.")
        )
        .arg(
            Arg::new("input")
                .short('i')
                .value_name("FILENAME")
                .help("read input data from FILENAME.")

        )
}

pub fn exec(matches: &ArgMatches) {

}
