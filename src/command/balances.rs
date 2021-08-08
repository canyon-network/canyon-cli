use anyhow::Result;
use structopt::StructOpt;
use subxt::balances::{LocksStoreExt, TransferCallExt, TransferEventExt};

use crate::{
    client::CanyonClient,
    runtime::{
        primitives::{AccountId, BlockNumber},
        CanyonSigner,
    },
    utils::parse_account,
};

/// Balances
#[derive(Debug, StructOpt)]
pub enum Balances {
    /// Transfer some balances from signer to another account.
    Transfer {
        /// receiver
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        dest: AccountId,
        /// amount
        #[structopt(index = 2)]
        value: u128,
    },
    /// Inspect the balances storage items.
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    /// Any liquidity locks on some account balances.
    Locks {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl Balances {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = CanyonClient::create(url).await?;

        match self {
            Balances::Transfer { dest, value } => {
                let result = client
                    .0
                    .transfer_and_watch(&signer, &dest.into(), value)
                    .await?;
                if let Some(event) = result.transfer()? {
                    println!("Balance transfer success: value: {:?}", event.amount);
                } else {
                    println!("Failed to find Balances::Transfer Event");
                }
            }
            Balances::Storage(storage) => match storage {
                Storage::Locks { who, block_number } => {
                    let at = client.block_hash(block_number).await?;
                    let locks = client.0.locks(&who, at).await?;
                    println!("{:?}: {:#?}", who, locks);
                }
            },
        }

        Ok(())
    }
}
