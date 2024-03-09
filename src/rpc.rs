//
// rpc.rs
//

use bitcoincore_rpc::{bitcoin, Auth, Client, Error, RpcApi};

// Creates an rpc client used as a base to communicate with the API methods
fn rpc(url: &str, user: &str, passwd: &str) -> Result<(), Error> {
    let url = url.to_string()
    let auth = Auth::UserPass(user.to_string(), passwd.to_string());
    Client::new(url, auth)
}
