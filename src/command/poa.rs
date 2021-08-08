use anyhow::Result;
use structopt::StructOpt;
use subxt::Store;

use pallet_poa::DepthInfo;

use crate::{
    client::CanyonClient,
    pallets::poa::{HistoryDepthStore, HistoryDepthStoreExt},
    runtime::{
        primitives::{AccountId, BlockNumber},
        CanyonRuntime, CanyonSigner,
    },
    utils::parse_account,
};

/// Poa
#[derive(Debug, StructOpt)]
pub enum Poa {
    /// Inspect the poa storage items.
    Storage(Storage),
}

#[derive(Debug, StructOpt)]
pub enum Storage {
    /// Retrieve the history depth of given account.
    HistoryDepth {
        /// Account
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
        /// Subscribe the storage changes of HistoryDepth.
        #[structopt(long)]
        watch: bool,
        /// Specify the state of given block number.
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
}

pub fn display_storage_ratio(depth_info: &DepthInfo<BlockNumber>) -> String {
    let storage_ratio = depth_info.as_storage_capacity();
    let percent_ratio = storage_ratio.deconstruct() as f64 / 1_000_000f64;
    format!("{}", percent_ratio)
}

impl Poa {
    pub async fn run(self, url: String, _signer: CanyonSigner) -> Result<()> {
        let client = CanyonClient::create(url).await?;

        match self {
            Self::Storage(storage) => match storage {
                Storage::HistoryDepth {
                    who,
                    block_number,
                    watch,
                } => {
                    if watch {
                        client.subscribe_poa_history_depth(&who).await?;
                    } else {
                        let at = client.block_hash(block_number).await?;
                        let key = HistoryDepthStore::<CanyonRuntime> { account_id: &who }
                            .key(client.metadata());
                        println!("key: {:?}", key);
                        let depth_info = client.0.history_depth(&who, at).await?;
                        println!("{:?}: {:#?}", who, depth_info);
                        println!(
                            "Estimated storage ratio: {:?}",
                            display_storage_ratio(&depth_info)
                        );
                    }
                }
            },
        }

        Ok(())
    }
}
