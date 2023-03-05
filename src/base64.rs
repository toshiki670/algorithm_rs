use clap::{Args, Subcommand};

mod encode;
// mod table;


#[derive(Args, Debug)]
#[command(version = "0.1.0")]
#[command(author)]
#[command(about = "base64 algorithm", long_about = None)]
pub struct Base64 {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Encode(encode::Encode),
}


pub fn exec(cli: &Base64) {
    match &cli.command {
        Commands::Encode(cli) => {
            encode::exec(cli);
        },
    }
}
