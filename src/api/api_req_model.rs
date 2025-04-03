use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignRequestModel {
   pub address: String,
   pub message: String,
   pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecoverRequestModel {
   pub address: String,
   pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VerifyRequestModel {
   pub address: String,
   pub message: String,
   pub signature: String,
}