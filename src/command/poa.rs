use anyhow::Result;
use structopt::StructOpt;

use crate::pallets::poa::HistoryDepthStoreExt;

use crate::{
    runtime::{
        primitives::{AccountId, BlockNumber},
        CanyonSigner,
    },
    utils::{block_hash, build_client, parse_account},
};

/// Poa
#[derive(Debug, StructOpt)]
pub enum Poa {
    /// Inspect the poa storage items.
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    /// Any liquidity locks on some account balances.
    HistoryDepth {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

impl Poa {
    pub async fn run(self, url: String, _signer: CanyonSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Storage(storage) => match storage {
                Storage::HistoryDepth { who, block_number } => {
                    let at = block_hash(&client, block_number).await?;
                    let depth_info = client.history_depth(&who, at).await?;
                    println!("{:?}: {:#?}", who, depth_info);
                    let storage_ratio = depth_info.as_storage_capacity();
                    let percent_ratio = storage_ratio.deconstruct() as f64 / 1_000_000f64;
                    println!("Estimated storage ratio: {:?}", percent_ratio);
                }
            },
        }

        Ok(())
    }
}
