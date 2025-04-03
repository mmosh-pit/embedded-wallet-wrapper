use std::collections::BTreeMap;
use frost_ed25519::{self as frost, keys::{PublicKeyPackage, SecretShare}};
use anyhow::{Ok, Result};

use super::frost_model::SignModel;


pub fn frost_create_signature(req: SignModel) -> Result<String> {
    let mut rng = rand::rngs::OsRng;
    let mut nonces_map = BTreeMap::new();
    let mut commitments_map = BTreeMap::new();
    let mut key_packages = BTreeMap::new();

    let pubkey_package  = PublicKeyPackage::deserialize(&hex::decode(req.clone().public_key_package).unwrap()).unwrap();

    for key_package_str in req.clone().key_package.into_iter().enumerate() {
        let key_hex = hex::decode(key_package_str.1).unwrap();
        let secret_share = SecretShare::deserialize(&key_hex).unwrap();
        let key_package = frost::keys::KeyPackage::try_from(secret_share).unwrap();
        let (nonces, commitments) = frost::round1::commit(
            key_package.signing_share(),
            &mut rng,
        );
        let identifier;
        if key_package_str.0 == 0 {
            identifier = frost::Identifier::derive(&"server".as_bytes()).unwrap()
        } else {
            identifier = frost::Identifier::derive(&"user".as_bytes()).unwrap()
        }
        key_packages.insert(identifier, key_package);
        nonces_map.insert(identifier, nonces);
        commitments_map.insert(identifier, commitments);
    }

    let mut signature_shares = BTreeMap::new();
    let message = hex::decode(req.clone().message).unwrap();
    let signing_package = frost::SigningPackage::new(commitments_map, &message);

    for participant_identifier in nonces_map.keys() {
        let key_package = &key_packages[participant_identifier];
        let nonces = &nonces_map[participant_identifier];
        let signature_share = frost::round2::sign(&signing_package, nonces, key_package).unwrap();
        signature_shares.insert(*participant_identifier, signature_share);
    }

    let group_signature = frost::aggregate(&signing_package, &signature_shares, &pubkey_package)?;

    let signature = hex::encode( group_signature.serialize().unwrap().to_vec());

    Ok(signature)
}