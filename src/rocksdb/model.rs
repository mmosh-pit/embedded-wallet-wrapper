use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletStore {
    pub public_key_package: String,
    pub key_package: String
}