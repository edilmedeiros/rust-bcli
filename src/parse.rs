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
    /// Specify configuration file. Relative paths will be prefixed by datadir location.
    #[arg(long, default_value = "")]
    pub conf: String,

    /// Send commands to the node running on <ip>
    #[arg(long, value_name = "ip")]
    pub rpcurl: Option<String>,

    /// Connect to JSON-RPC on <port>
    #[arg(long, value_name = "port")]
    pub rpcport: Option<String>,

    /// Username for JSON_RPC connections
    #[arg(long, value_name = "user")]
    pub rpcuser: Option<String>,

    /// Password for JSON-RPC connections
    #[arg(long, value_name = "pw")]
    pub rpcpassword: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    // man

    // stop
    #[command(name = GET_BEST_BLOCK_HASH, about = GET_BEST_BLOCK_HASH_MSG)]
    GetBestBlockHash,

    /// Returns the total uptime of the server.
    #[command(name = "uptime")]
    Uptime,
}

// Returns the arguments from the clap parser as an Args struct defined above
pub fn get_args() -> Opts {
    Opts::parse()
}
