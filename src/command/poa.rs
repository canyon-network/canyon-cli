use anyhow::Result;
use codec::Decode;
use jsonrpsee_types::{to_json_value, Subscription};
use structopt::StructOpt;
use subxt::{RpcClient, Store};

use pallet_poa::DepthInfo;
use sp_core::storage::StorageChangeSet;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};

use crate::{
    client::CanyonClient,
    pallets::poa::{HistoryDepthStore, HistoryDepthStoreExt},
    runtime::{
        primitives::{AccountId, BlockNumber, Hash},
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

struct PoaRpc<'a> {
    rpc: &'a RpcClient,
}

impl<'a> PoaRpc<'a> {
    pub fn new(rpc: &'a RpcClient) -> Self {
        Self { rpc }
    }

    /// Subscribe to System Events that are imported into blocks.
    ///
    /// *WARNING* these may not be included in the finalized chain, use
    /// `subscribe_finalized_events` to ensure events are finalized.
    pub async fn subscribe_history_depth(
        &self,
        who: &AccountId,
        client: &CanyonClient,
    ) -> Result<Subscription<StorageChangeSet<Hash>>> {
        let storage_key =
            HistoryDepthStore::<CanyonRuntime> { account_id: who }.key(client.metadata())?;

        let keys = Some(vec![storage_key]);

        let params = &[to_json_value(keys)?];

        let mut subscription = self
            .rpc
            .subscribe("state_subscribeStorage", params, "state_unsubscribeStorage")
            .await?;

        while let Ok(Some(StorageChangeSet { block, changes })) = subscription.next().await {
            if !changes.is_empty() {
                // We only subscribed one key.
                let (_storage_key, storage_data) = &changes[0];
                if let Some(data) = storage_data {
                    let new_depth_info: DepthInfo<BlockNumber> =
                        Decode::decode(&mut data.0.as_slice())?;
                    let number = client
                        .0
                        .block(Some(block))
                        .await?
                        .map(|chain_block| *chain_block.block.header().number())
                        .unwrap_or_default();
                    println!("number: {:?}", number);
                    println!(
                        "block #{}: {}, new_depth_info: {:?}, estimated storage ratio: {}",
                        number,
                        block,
                        new_depth_info,
                        display_storage_ratio(&new_depth_info)
                    );
                }
            }
        }

        Ok(subscription)
    }
}

fn display_storage_ratio(depth_info: &DepthInfo<BlockNumber>) -> String {
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
                        let poa_rpc = PoaRpc::new(client.rpc_client());
                        poa_rpc.subscribe_history_depth(&who, &client).await?;
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
