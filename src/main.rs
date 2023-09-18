use clap::Parser;
use std::error::Error;
mod utils;
use utils::install::install_bun;


#[derive(Parser)]
struct Cli {
    // command: use, install, default
    command: String ,
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args = Cli::parse();

  match args.command.as_ref() {
      "install" => install_bun(args.version),
      _ => Err("Invalid command")
  }
    
}