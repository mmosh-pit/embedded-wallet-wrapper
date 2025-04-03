use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateWalletModel {
    pub address: String,
    pub public_key_package: String,
    pub key_package: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignModel {
    pub message: String,
    pub public_key_package: String,
    pub key_package: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyModel {
    pub message: String,
    pub public_key_package: String,
    pub signature: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecoveryModel {
    pub public_key_package: String,
    pub key_package: Vec<String>
}