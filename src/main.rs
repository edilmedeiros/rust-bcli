//
// main.rs
//
#![allow(unused_imports)] // TODO: Remove when done
#![allow(unused_variables)] // TODO: Remove when done

// Project crates, only need to be imported in main
mod commands;
mod constants;
mod display;
mod parse;
mod rpc;
mod utils;

// Project shortcuts
use parse::Opts;
use parse::get_args;
use parse::Commands;
use commands::*;


// External crates shortcuts
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get the arguments from our wrapper parser
    let opts = get_args();

    // Get the rpc client from our rpc wrapper module
    let rpc = rpc::rpc(&opts.url, &opts.user, &opts.pass)?;

    // Match command args and call the right function!
    match opts.command {
        Commands::GetBestBlockHash => {
            get_best_block_hash(&rpc)?;
        }
        Commands::Uptime => {
            get_uptime(&rpc)?;
        }
    }

    Ok(())
}
