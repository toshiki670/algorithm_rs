use clap::{App, SubCommand};

use proconio::input;
mod n_triangle;

fn main() {
    let cli = clap::load_yaml!("cli.yml");
    let n_triangle_cli = clap::load_yaml!("n_triangle/cli.yml");

    let matches = App::from_yaml(cli)
        .subcommand(SubCommand::from_yaml(n_triangle_cli))
        .get_matches();


    println!("Select algorithms:");
    println!("0: n_triangle");

    println!("\nEnter number from 0 to 0:");
    input! {
        selected_num: u8,
    }

    println!();

    match selected_num {
        0 => n_triangle::exec(),
        1_u8..=u8::MAX => println!("Do nothing..."),
    }

    println!("\nGood bye!");
}
