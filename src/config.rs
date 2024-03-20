//
// config.rs
//

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub rpc_url: Option<String>,
    pub rpc_port: Option<String>,
    pub rpc_user: Option<String>,
    pub rpc_password: Option<String>,
}

// Defaults to None
impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: None,
            rpc_port: None,
            rpc_user: None,
            rpc_password: None,
        }
    }
}
