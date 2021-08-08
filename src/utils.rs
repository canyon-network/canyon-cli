use std::{fs::File, io::Read, path::Path};

use anyhow::{anyhow, Result};
use sp_core::crypto::{Pair, Public, Ss58Codec};
use sp_keyring::AccountKeyring;
use sp_runtime::traits::{IdentifyAccount, Verify};

use crate::runtime::primitives::{AccountId, Signature};

pub fn read_code<P: AsRef<Path>>(code_path: P) -> Result<Vec<u8>> {
    let mut file = File::open(code_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// Parses AccountId from String, also supports passing the test accounts directly.
pub fn parse_account(address: &str) -> Result<AccountId> {
    use AccountKeyring::*;
    match String::from(address).to_lowercase().as_str() {
        "alice" => Ok(Alice.to_account_id()),
        "bob" => Ok(Bob.to_account_id()),
        "charlie" => Ok(Charlie.to_account_id()),
        "dave" => Ok(Dave.to_account_id()),
        "eve" => Ok(Eve.to_account_id()),
        "ferdie" => Ok(Ferdie.to_account_id()),
        "one" => Ok(One.to_account_id()),
        "two" => Ok(Two.to_account_id()),
        _ => Ok(AccountId::from_string(address)
            .map_err(|err| anyhow!("Failed to parse account address: {:?}", err))?),
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}
