use anyhow::Result;
use codec::{Decode, Encode};
use jsonrpsee_types::{to_json_value, Subscription};
use subxt::{Client, ClientBuilder, Metadata, RpcClient, Store};

use sp_core::{storage::StorageChangeSet, Bytes, H256};
use sp_runtime::traits::{BlakeTwo256, Block as BlockT, Hash as HashT, Header as HeaderT};

use cp_permastore::CHUNK_SIZE;
use pallet_poa::DepthInfo;

use crate::{
    pallets::{permastore::StoreCallExt, poa::HistoryDepthStore},
    runtime::{
        primitives::{AccountId, BlockNumber, Hash},
        CanyonRuntime, CanyonSigner,
    },
};

/// Unit type wrapper of `Client` for Canyon runtime.
#[derive(Clone)]
pub struct CanyonClient(pub Client<CanyonRuntime>);

impl CanyonClient {
    /// Creates a new instance of Canyon client.
    pub async fn create<U: Into<String>>(url: U) -> Result<Self> {
        let client = ClientBuilder::<CanyonRuntime>::new()
            .set_url(url)
            .skip_type_sizes_check()
            .build()
            .await?;
        Ok(Self(client))
    }

    /// Returns the genesis hash.
    pub fn genesis(&self) -> &Hash {
        self.0.genesis()
    }

    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        self.0.metadata()
    }

    /// Returns the rpc client.
    pub fn rpc_client(&self) -> &RpcClient {
        self.0.rpc_client()
    }

    /// Get the block hash given block number.
    pub async fn block_hash(&self, block_number: Option<BlockNumber>) -> Result<Option<Hash>> {
        if let Some(number) = block_number {
            Ok(self.0.block_hash(Some(number.into())).await?)
        } else {
            Ok(None)
        }
    }

    /// Returns the block number given block hash.
    pub async fn block_number(&self, block_hash: Hash) -> Result<Option<BlockNumber>> {
        Ok(self
            .0
            .block(Some(block_hash))
            .await?
            .map(|signed_block| *signed_block.block.header().number()))
    }

    /// Send `permastore::call` extrinsic.
    pub async fn store(&self, signer: &CanyonSigner, data: Vec<u8>) -> Result<()> {
        let chunks = data
            .chunks(CHUNK_SIZE as usize)
            .map(|c| BlakeTwo256::hash(c).encode())
            .collect();

        let chunk_root = BlakeTwo256::ordered_trie_root(chunks);
        let data_size = data.len() as u32;
        println!("data size: {:?}, chunk root: {:?}", data_size, chunk_root);

        let result = self.0.store(signer, data_size, chunk_root).await?;
        println!("Stored result: {:?}", result);

        Ok(())
    }
}

////    RPC implementations
impl CanyonClient {
    ///////////////////////////////////////////////////////////////////////
    ////    Permastore
    ///////////////////////////////////////////////////////////////////////
    /// Submit the transaction data.
    pub async fn permastore_submit(&self, value: Bytes) -> Result<H256> {
        let params = &[to_json_value(value)?];
        let data = self
            .rpc_client()
            .request("permastore_submit", params)
            .await?;
        Ok(data)
    }

    /// Submit the `store` extrinsic as well as the transaction data.
    pub async fn permastore_submit_extrinsic(&self, value: Bytes, data: Bytes) -> Result<H256> {
        let params = &[to_json_value(value)?, to_json_value(data)?];
        let data = self
            .rpc_client()
            .request("permastore_submitExtrinsic", params)
            .await?;
        Ok(data)
    }

    /// Remove the transaction data given chunk root.
    pub async fn permastore_remove_data(&self, chunk_root: Hash) -> Result<bool> {
        let params = &[to_json_value(chunk_root)?];
        let data = self
            .rpc_client()
            .request("permastore_removeData", params)
            .await?;
        Ok(data)
    }

    ///////////////////////////////////////////////////////////////////////
    ////    Poa
    ///////////////////////////////////////////////////////////////////////
    async fn watch_poa_history_depth(
        &self,
        who: &AccountId,
    ) -> Result<Subscription<StorageChangeSet<Hash>>> {
        let storage_key =
            HistoryDepthStore::<CanyonRuntime> { account_id: who }.key(self.metadata())?;

        let keys = Some(vec![storage_key]);

        let params = &[to_json_value(keys)?];

        let subscription = self
            .rpc_client()
            .subscribe("state_subscribeStorage", params, "state_unsubscribeStorage")
            .await?;

        Ok(subscription)
    }

    /// Subscribe to System Events that are imported into blocks.
    ///
    /// *WARNING* these may not be included in the finalized chain, use
    /// `subscribe_finalized_events` to ensure events are finalized.
    pub async fn subscribe_poa_history_depth(
        &self,
        who: &AccountId,
    ) -> Result<Subscription<StorageChangeSet<Hash>>> {
        let mut subscription = self.watch_poa_history_depth(who).await?;

        while let Ok(Some(StorageChangeSet { block, changes })) = subscription.next().await {
            if !changes.is_empty() {
                // We only subscribed one key.
                let (_storage_key, storage_data) = &changes[0];
                if let Some(data) = storage_data {
                    let new_depth_info: DepthInfo<BlockNumber> =
                        Decode::decode(&mut data.0.as_slice())?;
                    let number = self.block_number(block).await?.unwrap_or_default();
                    println!(
                        "block #{}: {}, new_depth_info: {:?}, estimated storage ratio: {}",
                        number,
                        block,
                        new_depth_info,
                        crate::command::poa::display_storage_ratio(&new_depth_info)
                    );
                }
            }
        }

        Ok(subscription)
    }
}
