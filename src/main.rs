#![forbid(unsafe_code)]

use std::process::ExitCode;

use clap::Parser;
use zdev::{Cli, render_error, render_success, run};

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(output) => {
            render_success(cli.format(), &output);
            ExitCode::from(output.exit_code)
        }
        Err(error) => {
            render_error(cli.format(), cli.command_name(), &error);
            ExitCode::from(2)
        }
    }
}
