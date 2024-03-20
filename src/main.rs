//
// main.rs
//
#![allow(unused_imports)] // TODO: Remove when done
#![allow(unused_variables)] // TODO: Remove when done

// Project crates, only need to be imported in main
mod commands;
mod config;
mod constants;
mod display;
mod parse;
mod rpc;
mod utils;

// Project shortcuts
use commands::*;
use commands::*;
use config::Config;
use confy;
use parse::get_args;
use parse::Command;
use parse::Opts;

// External crates shortcuts
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Where should be the default folder and format for bcli config?
    let config: Config = confy::load("bcli", "config")?;

    // Get the arguments from our wrapper parser
    let opts = get_args();

    // If there's a configuration file we use the rpc values from there.
    let url: String = match config.rpc_url {
        Some(url) => url,
        None => opts
            .rpcurl
            .expect("You should provide RPC url by configuration file or args. Use --help"),
    };

    let port: String = match config.rpc_port {
        Some(pass) => pass,
        None => opts
            .rpcpassword
            .expect("You should provide RPC pass by configuration file or args. Use --help"),
    };

    let user: String = match config.rpc_user {
        Some(user) => user,
        None => opts
            .rpcuser
            .expect("You should provide RPC user by configuration file or args. Use --help"),
    };

    let pass: String = match config.rpc_password {
        Some(pass) => pass,
        None => opts
            .rpcport
            .expect("You should provide RPC pass by configuration file or args. Use --help"),
    };

    // Get the rpc client from our rpc wrapper module
    let rpc = rpc::rpc(&url, &port, &user, &pass)?;

    // Match command args and call the right function!
    match opts.command {
        Command::GetBestBlockHash => {
            get_best_block_hash(&rpc)?;
        }
        Command::Uptime => {
            get_uptime(&rpc)?;
        }
    }

    Ok(())
}
