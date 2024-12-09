// https://github.com/clap-rs/clap/blob/master/clap_complete/examples/completion-derive.rs

use std::io::stdout;

use crate::cli::{CliArgs, Route};
use clap::{Args, CommandFactory};
use clap_complete::{generate, Generator, Shell};

#[derive(Args, Debug)]
#[command(about = "System commands")]
pub struct SystemArgs {
    #[arg(value_enum, long, help = "Spawn shell completions")]
    completions_shell: Shell,
}

impl Route for SystemArgs {
    fn route(&self) {
        let shell = self.completions_shell;
        generate_completions(shell);
    }
}

fn generate_completions<G: Generator>(gen: G) {
    let mut cmd = CliArgs::command();
    let bin_name = cmd.get_name().to_string();
    generate(gen, &mut cmd, bin_name, &mut stdout());
}
