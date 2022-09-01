use std::process;

#[macro_use]
extern crate scan_fmt;
mod cli;
mod config;
mod container;
mod errors;
mod ipc;

use errors::exit_with_retcode;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(Ok(()));
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            process::exit(e.get_retcode());
        }
    };
}
