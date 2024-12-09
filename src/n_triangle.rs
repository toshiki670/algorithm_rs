mod calc;

use crate::cli::Route;
use clap::Args;

#[derive(Args, Debug)]
#[command(name = "n_triangle")]
#[command(version = "1.3.0")]
#[command(author)]
#[command(about = "n_triangle calcurater", long_about = None)]
pub struct NTriangleArgs {
    pub height: u32,
}

impl Route for NTriangleArgs {
    fn route(&self) {
        let height = self.height;
        println!(
            "The answer with a height of {} is {}.",
            height,
            calc::call(height)
        );
    }
}
