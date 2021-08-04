use std::path::PathBuf;

use anyhow::{anyhow, Result};
use jsonrpsee_types::to_json_value;
use sp_core::{Bytes, Encode, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};
use structopt::StructOpt;
use subxt::RpcClient;

use cp_permastore::CHUNK_SIZE;

use crate::{
    runtime::{pallets::permastore::StoreCallExt, CanyonClient, CanyonSigner},
    utils::build_client,
};

/// Permastore
#[derive(Debug, StructOpt)]
pub enum Permastore {
    /// Send the `store` extrinsic.
    Store {
        #[structopt(long)]
        data: Option<String>,
        #[structopt(short, long, parse(from_os_str), conflicts_with = "data")]
        path: Option<PathBuf>,
    },
    /// Submit the transction data via RPC.
    Submit {
        #[structopt(long)]
        data: Option<String>,
        #[structopt(short, long, parse(from_os_str), conflicts_with = "data")]
        path: Option<PathBuf>,
    },
    /// Combine `Submit` and `Store`
    SubmitAndStore {
        #[structopt(long)]
        data: Option<String>,
        #[structopt(short, long, parse(from_os_str), conflicts_with = "data")]
        path: Option<PathBuf>,
    },
}

/// Send `permastore::call` extrinsic.
async fn store(client: &CanyonClient, signer: &CanyonSigner, data: Vec<u8>) -> Result<()> {
    let chunks = data
        .chunks(CHUNK_SIZE as usize)
        .map(|c| BlakeTwo256::hash(c).encode())
        .collect();

    let chunk_root = BlakeTwo256::ordered_trie_root(chunks);
    let data_size = data.len() as u32;
    println!("data size: {:?}, chunk root: {:?}", data_size, chunk_root);

    let result = client.store(signer, data_size, chunk_root).await?;
    println!("Stored result: {:?}", result);

    Ok(())
}

struct PermastoreRpc<'a> {
    rpc: &'a RpcClient,
}

impl<'a> PermastoreRpc<'a> {
    pub fn new(rpc: &'a RpcClient) -> Self {
        Self { rpc }
    }

    /// Submit the transaction data.
    pub async fn submit(&self, value: Bytes) -> Result<H256> {
        let params = &[to_json_value(value)?];
        let data = self.rpc.request("permastore_submit", params).await?;
        Ok(data)
    }

    /// Submit the `store` extrinsic as well as the transaction data.
    async fn submit_extrinsic(&self, value: Bytes, data: Bytes) -> Result<H256> {
        let params = &[to_json_value(value)?, to_json_value(data)?];
        let data = self
            .rpc
            .request("permastore_submitExtrinsic", params)
            .await?;
        Ok(data)
    }
}

impl Permastore {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = build_client(url).await?;

        let read_data = |data: Option<String>, path: Option<PathBuf>| {
            if let Some(data) = data {
                Ok(data.as_bytes().to_vec())
            } else if let Some(path) = path {
                std::fs::read(path).map_err(Into::into)
            } else {
                Err(anyhow!("--data or --path is required for store command"))
            }
        };

        match self {
            Self::Store { data, path } => {
                let raw_data = read_data(data, path)?;
                store(&client, &signer, raw_data).await?;
            }
            Self::Submit { data, path } => {
                let raw_data = read_data(data, path)?;
                let permastore_rpc = PermastoreRpc::new(client.rpc_client());
                let ret = permastore_rpc.submit(raw_data.into()).await?;
                println!("Submitted result: {:?}", ret);
            }
            Self::SubmitAndStore { data, path } => {
                let data = read_data(data, path)?;

                let chunks = data
                    .chunks(CHUNK_SIZE as usize)
                    .map(|c| BlakeTwo256::hash(c).encode())
                    .collect();

                let chunk_root = BlakeTwo256::ordered_trie_root(chunks);
                let data_size = data.len() as u32;
                println!("data size: {:?}, chunk root: {:?}", data_size, chunk_root);

                let store_call =
                    crate::runtime::pallets::permastore::StoreCall::new(data_size, chunk_root);
                let uxt = client.create_signed(store_call, &signer).await?;

                let permastore_rpc = PermastoreRpc::new(client.rpc_client());
                let ret = permastore_rpc
                    .submit_extrinsic(uxt.encode().into(), data.into())
                    .await?;
                println!("Submitted result: {:?}", ret);
            }
        }

        Ok(())
    }
}
