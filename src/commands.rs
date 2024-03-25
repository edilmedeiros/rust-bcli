//
// commands.rs
//
// TODO: Put here implementations of all the commands
//
#![allow(unused_imports)] // TODO: Remove when done
#![allow(unused_variables)] // TODO: Remove when done

// Project shortcuts
use crate::constants::*;
use crate::utils::*;

// External crates shortcuts
use bitcoincore_rpc::{bitcoin, bitcoin::BlockHash, Auth, Client, Error, RpcApi};

// Takes the rpc client as an input. We use a reference to avoid rust's
// default move strategy in order to reuse the client elsewhere as well.
// The function is expected to return the best block as a primitive number
// type (Not implemented yet)

pub fn stop_node(rpc: &Client) -> Result<String, Error> {
    let result = rpc.stop()?;
    assert_eq!(result, "Bitcoin Core stopping");
    println!("{result}");
    Ok(result)
}

pub fn get_best_block_hash(rpc: &Client) -> Result<BlockHash, Error> {
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);
    Ok(best_block_hash)
}

pub fn get_uptime(rpc: &Client) -> Result<String, Error> {
    let uptime_seconds = rpc.uptime()?;
    let formatted_uptime = format_uptime(uptime_seconds);
    println!("uptime: {}", &formatted_uptime);
    Ok(formatted_uptime)
}

