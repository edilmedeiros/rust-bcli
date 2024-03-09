extern crate bitcoincore_rpc;

use clap::Parser;
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {

    #[arg(long)]
    url: String,

    #[arg(short, long)]
    user: String,

    #[arg(short, long)]
    pass: String,


}


fn main() {
    let args = Args::parse();

    let rpc = Client::new(&args.url, Auth::UserPass(args.user, args.pass)).unwrap();

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

}