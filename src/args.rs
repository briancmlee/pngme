use std::path::PathBuf;
use clap::{Parser, Subcommand, Args};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs)
}

#[derive(Args)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String
}

#[derive(Args)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

#[derive(Args)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String
}

#[derive(Args)]
pub struct PrintArgs {
    pub file_path: PathBuf
}