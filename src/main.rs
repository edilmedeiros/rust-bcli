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
use commands::*;
use parse::get_args;
use parse::Args;
use parse::Commands;

// External crates shortcuts
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the arguments from our wrapper parser
    let args = get_args();

    // Get the rpc client from our rpc wrapper module
    let rpc = rpc::rpc(&args.url, &args.user, &args.pass)?;

    match args.command {
        Commands::GetBestBlockHash => {
            let best_block_hash = get_best_block_hash(&rpc)?;
            println!("Best block hash: {}", best_block_hash);
        }
        // Example for handling a hypothetical Uptime command
        Commands::Uptime => {
            let uptime = get_uptime(&rpc)?;
        }
    }

    /***
    Left here as reference from the example at:
    https://github.com/rust-bitcoin/rust-bitcoincore-rpc/blob/1b51e3d0bb614d36d256947f55d228ac0e1dc58f/client/examples/test_against_node.rs

    let _blockchain_info = rpc.get_blockchain_info();
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("best block hash: {}", best_block_hash);
    let bestblockcount = rpc.get_block_count().unwrap();
    println!("best block height: {}", bestblockcount);
    let best_block_hash_by_height = rpc.get_block_hash(bestblockcount).unwrap();
    println!("best block hash by height: {}", best_block_hash_by_height);
    assert_eq!(best_block_hash_by_height, best_block_hash);
    let bitcoin_block: bitcoin::Block = rpc.get_by_id(&best_block_hash).unwrap();
    println!("best block hash by `get`: {}", bitcoin_block.header.prev_blockhash);
    let bitcoin_tx: bitcoin::Transaction = rpc.get_by_id(&bitcoin_block.txdata[0].txid()).unwrap();
    println!("tx by `get`: {}", bitcoin_tx.txid());
    */

    Ok(())
}
