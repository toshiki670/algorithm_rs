use clap::{App, SubCommand};

mod n_triangle;

fn main() {
    let cli = clap::load_yaml!("cli.yml");
    let n_triangle_cli = clap::load_yaml!("n_triangle/cli.yml");

    let matches = App::from_yaml(cli)
        .subcommand(SubCommand::from_yaml(n_triangle_cli))
        .get_matches();


    if let Some(m) = matches.subcommand_matches("n_triangle") {
        n_triangle::exec(m);
    } else {
        println!("Do nothing...");
    }

    println!("\nGood bye!");
}
