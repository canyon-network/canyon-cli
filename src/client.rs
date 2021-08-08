use anyhow::Result;
use codec::Encode;
use subxt::{Client, ClientBuilder, Metadata, RpcClient};

use sp_runtime::traits::{BlakeTwo256, Hash as HashT};

use cp_permastore::CHUNK_SIZE;

use crate::pallets::permastore::StoreCallExt;
use crate::runtime::{
    primitives::{BlockNumber, Hash},
    CanyonRuntime, CanyonSigner,
};

/// Canyon `Client` for Canyon runtime.
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
        &self.0.genesis()
    }

    /// Returns the chain metadata.
    pub fn metadata(&self) -> &Metadata {
        &self.0.metadata()
    }

    /// Returns the rpc client.
    pub fn rpc_client(&self) -> &RpcClient {
        &self.0.rpc_client()
    }

    /// Get a block hash. By default returns the latest block hash
    pub async fn block_hash(&self, block_number: Option<BlockNumber>) -> Result<Option<Hash>> {
        if let Some(number) = block_number {
            Ok(self.0.block_hash(Some(number.into())).await?)
        } else {
            Ok(None)
        }
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
