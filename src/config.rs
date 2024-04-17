//
// config.rs
//

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub rpc_url: Option<String>,
    pub rpc_port: Option<String>,
    pub rpc_user: Option<String>,
    pub rpc_password: Option<String>,
}
