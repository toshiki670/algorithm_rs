use clap::{Command, ArgMatches};

mod encode;
// mod table;

pub fn cli() -> Command {
    Command::new("base64")
        .version("0.1.0")
        .author("Toshiki <bushy.trivial.0o@icloud.com>")
        .about("base64 algorithm")
        .subcommand_required(true)
        .subcommand(encode::cli())
}

pub fn exec(matches: &ArgMatches) {
    match matches.subcommand() {
        Some(("enc", sub_matches)) => {
            encode::exec(sub_matches);
        }
        _ => unreachable!()
    }
}
