mod validator;

use clap::{CommandFactory, Parser, Subcommand};

use coral_lib::error::AppResult;

use crate::print_version;

use self::validator::ValidatorCommand;

#[derive(Clone, Debug, Parser)]
#[command(author, about, arg_required_else_help(true))]
pub struct CommandArgs {
    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    #[arg(short = 'v', long = "version")]
    pub version: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SubCommand {
    #[command(about = "Show shell completions")]
    Completions { shell: clap_complete::Shell },

    #[command(subcommand, about = "Validator commands", name = "validator")]
    Validator(ValidatorCommand),

    #[command(about = "Show version")]
    Version,
}

pub async fn run_command(command: SubCommand) -> AppResult<i32> {
    match command {
        SubCommand::Version => {
            print_version();
            Ok(0)
        }
        SubCommand::Completions { shell } => {
            let mut app = CommandArgs::command();
            let bin_name = app.get_name().to_string();
            clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
            Ok(0)
        }
        SubCommand::Validator(subcommand) => subcommand.execute().await,
    }
}
