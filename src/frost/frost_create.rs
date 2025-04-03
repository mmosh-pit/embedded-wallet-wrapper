use anyhow::{Ok, Result};
use frost_ed25519 as frost;
use bs58;
use std::convert::TryInto;
use super::frost_model::CreateWalletModel;

pub fn frost_create_wallet() -> Result<Vec<CreateWalletModel>> {
    let mut rng = rand::rngs::OsRng;
    let (shares, pubkey_package) = frost::keys::generate_with_dealer(
        3,
        2,
        frost::keys::IdentifierList::Custom(&[frost::Identifier::derive("user".as_bytes())?, frost::Identifier::derive("recover".as_bytes())?, frost::Identifier::derive("server".as_bytes())?]),
        &mut rng,
    )?;
    let ed25519_pubkey: [u8; 32] = pubkey_package.verifying_key().serialize().unwrap().try_into().expect("Vec<u8> must have exactly 32 bytes") ;
    let public_key_package = hex::encode(pubkey_package.serialize().unwrap());
    let address = bs58::encode(ed25519_pubkey).into_string();

    let mut result = Vec::new();

    for share_item  in shares {
        result.push(CreateWalletModel {
            address: address.clone(),
            public_key_package: public_key_package.clone(),
            key_package: hex::encode(share_item.1.serialize().unwrap())
        });
    }
    
   Ok(result)
}