use args::{Cli, Commands};
use commands::{
    encode,
    decode,
    remove,
    print
};
use clap::Parser;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

// pub type Error = Box<dyn std::error::Error>;
// pub type Result<T> = std::result::Result<T, Error>;
pub type Error = anyhow::Error;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encode(args) => encode(args),
        Commands::Decode(args) => decode(args),
        Commands::Remove(args) => remove(args),
        Commands::Print(args) => print(args),
    }
}
