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

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::Value;

// Project shortcuts
use commands::*;
use commands::*;
use config::Config;
use parse::get_args;
use parse::Command;
use parse::Opts;

// External crates shortcuts
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the arguments from our wrapper parser
    let opts = get_args();

    // Check for provided file path or use config dirs
    let bcli_config: PathBuf = match opts.conf {
        Some(conf) => PathBuf::from(conf),
        None => dirs::config_dir()
            .ok_or_else(|| "Could not get config dir")?
            .join("Bitcoin/bcli.toml"),
    };

    let mut file = File::open(bcli_config).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read file");

    let config: Config = toml::from_str(&contents).expect("Could not parse TOML");

    // If there's a configuration file we use the rpc values from there.
    let url: String = match opts.rpcurl {
        Some(url) => url,
        None => config
            .rpc_url
            .expect("You should provide RPC url by configuration file or args. Use --help"),
    };

    let port: String = match opts.rpcpassword {
        Some(pass) => pass,
        None => config
            .rpc_port
            .expect("You should provide RPC pass by configuration file or args. Use --help"),
    };

    let user: String = match opts.rpcuser {
        Some(user) => user,
        None => config
            .rpc_user
            .expect("You should provide RPC user by configuration file or args. Use --help"),
    };

    let pass: String = match opts.rpcport {
        Some(pass) => pass,
        None => config
            .rpc_password
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
