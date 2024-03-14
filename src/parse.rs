//
// parse.rs
//
// TODO: I assume that further parsing functions are expected and they can be
// placed here as well
//
#![allow(unused_imports)] // TODO: Remove when done

// Priject shortcuts
use crate::constants::*;

// External crates shortcuts
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub url: String,

    #[arg(short, long)]
    pub user: String,

    #[arg(short, long)]
    pub pass: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = GET_BEST_BLOCK_HASH, about = GET_BEST_BLOCK_HASH_MSG)]
    GetBestBlockHash,

    #[command(name = UPTIME, about = UPTIME)]
    Uptime,
}

// Returns the arguments from the clap parser as an Args struct defined above
pub fn get_args() -> Args {
    Args::parse()
}
