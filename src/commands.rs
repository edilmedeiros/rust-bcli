//
// commands.rs
//
// TODO: Put here implementations of all the commands
//
#![allow(unused_imports)] // TODO: Remove when done
#![allow(unused_variables)] // TODO: Remove when done

// Project shortcuts
use crate::constants::*;

// External crates shortcuts
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

// Takes the rpc client as an input. We use a reference to avoid rust's
// default move strategy in order to reuse the client elsewhere as well.
// The function is expected to return the best block as a primitive number 
// type (Not implemented yet) 
pub fn get_best_block_hash(rpc: &Client) {
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("best block hash: {}", best_block_hash);
    println!("{}", GET_BEST_BLOCK_HASH);
}
