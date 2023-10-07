use clap::Parser;
mod utils;
use utils::{command::use_bun, remove_bun, display_versions_list};
use owo_colors::{DynColors, OwoColorize};
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Opts {
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

#[derive(Parser)]
pub struct ListCommand{

}

#[tokio::main]
pub async fn main() {
    let opts = Opts::try_parse();
    
    match opts {
      Ok(result) => {
        if result.version {
          println!("{}", VERSION);
        } else {
          match result.command {
            Some(command) =>  {
              match command {
                  Command::Default(_args) => {
                    println!("This feature will be implemented in the future.");
                  },
                  Command::Remove(args) => {
                    remove_bun(&args.version).await;
                  }
                  Command::Use(args) => {
                    let _ = use_bun(&args.version).await;
                  },
                  Command::List(_args) => {
                    display_versions_list()
                  }
              } 
            },
            None => {
                println!("Use -h to print help")
            }
          }
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
        "#f6e0b5", "#aa6f73", "#eea990",
    ]
    .map(|color| color.parse().unwrap());  

    for line in BUM.split_inclusive('\n') {
        
        print!("{}", line[0..16].to_string().color(colors[0]));
        print!("{}", line[17..33].to_string().color(colors[1]));
        print!("{}", line[34..56].to_string().color(colors[2]));

        
    }
    println!()                              
}