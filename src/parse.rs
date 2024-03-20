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
pub struct Opts {

    #[arg(long, default_value = "")]
    pub conf: String,

    #[arg(long, default_value = "localhost")]
    pub rpcurl: String,

    #[arg(long, default_value = "8332")]
    pub rpcport: String,

    #[arg(long, default_value = "")]
    pub rpcuser: String,

    #[arg(long, default_value = "")]
    pub rpcpassword: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {

    // man

    // stop

    #[command(name = GET_BEST_BLOCK_HASH, about = GET_BEST_BLOCK_HASH_MSG)]
    GetBestBlockHash,

    #[command(name = UPTIME, about = UPTIME)]
    Uptime,

}

// Returns the arguments from the clap parser as an Args struct defined above
pub fn get_args() -> Opts {
    Opts::parse()
}
