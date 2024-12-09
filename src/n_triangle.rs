mod calc;

use crate::cli::CliRoute;
use clap::Args;

#[derive(Args, Debug)]
#[command(name = "n_triangle")]
#[command(version = "1.3.0")]
#[command(author)]
#[command(about = "n_triangle calcurater", long_about = None)]
pub struct NTriangle {
    pub height: u32,
}

impl CliRoute for NTriangle {
    fn route(&self) {
        let height = self.height;
        println!(
            "The answer with a height of {} is {}.",
            height,
            calc::call(height)
        );
    }
}
