use clap::Args;

mod calc;

#[derive(Args, Debug)]
#[command(name = "n_triangle")]
#[command(version = "1.2.0")]
#[command(author)]
#[command(about = "n_triangle calcurater", long_about = None)]
pub struct NTriangle {
    pub height: u32,
}


pub fn exec(cli: &NTriangle) {
    let height = cli.height;

    println!("The answer with a height of {} is {}.", height, calc::call(height));
}
