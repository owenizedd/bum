#![allow(clippy::all, clippy::pedantic)]

use anyhow::Result;
use clap::Parser;
use commands::{list, list_remote, remove, use_bumrc, use_bun};
use owo_colors::{DynColors, OwoColorize};

mod bun;
mod commands;
mod os;
mod utils;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[clap(short, long)]
    version: bool,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser)]
pub enum Command {
    Default(DefaultCommand),
    Use(UseCommand),
    Remove(RemoveCommand),
    List(ListCommand),
    ListRemote(ListRemoteCommand),
}

#[derive(Parser)]
pub struct DefaultCommand {
    version: String,
}

#[derive(Parser)]
pub struct UseCommand {
    version: Option<String>,
}

#[derive(Parser)]
pub struct RemoveCommand {
    version: String,
}

#[derive(Parser)]
pub struct ListCommand {}
#[derive(Parser)]
pub struct ListRemoteCommand {}

#[tokio::main]
pub async fn main() {
    let cli = Cli::try_parse();

    match cli {
        Ok(result) => {
            if result.version {
                println!("{}", VERSION);
            } else {
                if let Err(e) = run_commands(result.command).await {
                    println!("An error occured during the execution: {e}");
                };
            }
        }
        Err(e) => {
            print_default_message();
            println!("{}", e);
        }
    }
}

async fn run_commands(used_command: Option<Command>) -> Result<()> {
    if used_command.is_none() {
        println!("Use -h to print help");

        return Ok(());
    }
    match used_command.unwrap() {
        Command::Default(_args) => {
            println!("This feature will be implemented in the future.");
        }
        Command::Remove(args) => {
            remove(&args.version).await;
        }
        Command::Use(args) => match args.version {
            Some(version) => {
                use_bun(&version).await?;
            }
            None => {
                use_bumrc().await?;
            }
        },
        Command::List(_) => list().await?,
        Command::ListRemote(_) => {
            list_remote().await;
        }
    }

    Ok(())
}

fn print_default_message() {
    const BUM: &str = r#"         _____    ____   ____      ______  _______   
    ___|\     \  |    | |    |    |      \/       \  
   |    |\     \ |    | |    |   /          /\     \ 
   |    | |     ||    | |    |  /     /\   / /\     |
   |    | /_ _ / |    | |    | /     /\ \_/ / /    /|
   |    |\    \  |    | |    ||     |  \|_|/ /    / |
   |    | |    | |    | |    ||     |       |    |  |
   |____|/____/| |\___\_|____||\____\       |____|  /
   |    /     || | |    |    || |    |      |    | / 
   |____|_____|/  \|____|____| \|____|      |____|/  
     \(    )/        \(   )/      \(          )/     
      '    '          '   '        '          '       "#;

    let colors: [DynColors; 3] =
        ["#f6e0b5", "#aa6f73", "#eea990"].map(|color| color.parse().unwrap());

    for line in BUM.split_inclusive('\n') {
        print!("{}", line[0..16].to_string().color(colors[0]));
        print!("{}", line[17..33].to_string().color(colors[1]));
        print!("{}", line[34..54].to_string().color(colors[2]));
    }
    println!()
}
