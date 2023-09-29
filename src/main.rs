use clap::Parser;
use std::error::Error;
mod utils;
use utils::command::use_bun;
use owo_colors::{DynColors, OwoColorize};


#[derive(Parser)]
pub enum Opts {
    Default(DefaultCommand),
    Use(UseCommand),
    Remove(RemoveCommand),
}

#[derive(Parser)]
pub struct DefaultCommand {
    version: String
}

#[derive(Parser)]
pub struct UseCommand {
    version: String
}

#[derive(Parser)]
pub struct RemoveCommand {
    version: String
}


#[tokio::main]
pub async fn main() {

    match Opts::try_parse() {
      Ok(command) => match command {
        Opts::Default(args) => {

        },
        Opts::Remove(args) => {

        }
        Opts::Use(args) => {
           let _ = use_bun(&args.version).await;
        }

      },
      Err(e) => {
        print_default_message();
        println!("{}", e);
      }
    }
}


fn print_default_message() {
    const BUM: &str = r#"      _____    ____   ____      ______  _______        
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

    let colors: [DynColors; 3] = [
        "#B80A41", "#4E4BA8", "#6EB122",
    ]
    .map(|color| color.parse().unwrap());  

    for line in BUM.split_inclusive('\n') {
        
        print!("{}", line[0..16].to_string().color(colors[0]));
        print!("{}", line[17..32].to_string().color(colors[1]));
        print!("{}", line[32..56].to_string().color(colors[2]));

        
    }
    println!()                              
}