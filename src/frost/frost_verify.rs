
use frost_ed25519::{keys::PublicKeyPackage, Signature};
use super::frost_model::VerifyModel;


pub fn frost_verify_signature(req: VerifyModel) -> bool {
    let pubkey_package  = PublicKeyPackage::deserialize(&hex::decode(req.clone().public_key_package).unwrap()).unwrap();
    let message = hex::decode(req.clone().message).unwrap();
    let group_signature = Signature::deserialize(&hex::decode( req.clone().signature).unwrap()).unwrap();
    pubkey_package
    .verifying_key()
    .verify(&message, &group_signature)
    .is_ok()
}