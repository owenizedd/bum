use clap::Parser;
use std::error::Error;
mod utils;
use utils::command::use_bun;


#[derive(Parser)]
struct Cli {
    command: String,
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let args = Cli::parse();

  match args.command.as_ref() {
    "use" => {
        use_bun(&args.version).await?;
    }
    _ => {
        return Err("Unknown command".into());
    }
}

  Ok(())
    
}