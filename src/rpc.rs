//
// rpc.rs
//
// TODO: Include functions that get the user name and password from the 
// bitcoin.conf file
//
#![allow(unused_imports)] // TODO: Remove when done

// External crates shortcuts
use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

// Creates an rpc client used as a base to communicate with the API methods
pub fn rpc(url: &str, user: &str, passwd: &str) -> Result<Client, Error> {
    let auth = Auth::UserPass(user.to_string(), passwd.to_string());
    Client::new(url, auth)
}
