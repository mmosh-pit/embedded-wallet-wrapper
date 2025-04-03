use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateWalletResponse {
    pub address: String,
    pub key_package: Vec<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseModel {
   pub status: bool,
   pub message: String,
   pub data: Option<String> 
}