mod commands;
use std::process;

use clap::Parser;
use colored::*;

use ethers::contract::abigen;

use coral_lib::error::AppResult;

use crate::commands::CommandArgs;

pub const PROGRAM_NAME: &str = "coral-cli";

abigen!(PufferOracle, "./abi/PufferOracle.json");
abigen!(PufferProtocol, "./abi/PufferProtocol.json");
abigen!(ValidatorTicket, "./abi/ValidatorTicket.json");

pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn print_version() {
    println!("{PROGRAM_NAME}-{APP_VERSION}");
}

async fn run_main(args: CommandArgs) -> AppResult<i32> {
    if args.version {
        print_version();
        return Ok(0);
    }

    match args.subcommand {
        Some(command) => commands::run_command(command).await,
        None => Ok(0),
    }
}

#[tokio::main]
async fn main() {
    let args = CommandArgs::parse();

    match run_main(args).await {
        Ok(exit_code) => process::exit(exit_code),
        Err(err) => {
            let err_msg = format!("{}", err).red();
            eprintln!("{}", err_msg);
            process::exit(1);
        }
    }
}
