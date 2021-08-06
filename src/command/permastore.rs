use std::path::PathBuf;

use anyhow::{anyhow, Result};
use jsonrpsee_types::to_json_value;
use sp_core::{Bytes, Encode, H256};
use sp_runtime::traits::{BlakeTwo256, Hash};
use structopt::StructOpt;
use subxt::RpcClient;

use cp_permastore::CHUNK_SIZE;

use crate::{
    pallets::permastore::StoreCallExt,
    runtime::{CanyonClient, CanyonSigner},
    utils::build_client,
};

#[derive(Debug, StructOpt)]
pub struct SharedParams {
    /// Raw data to upload.
    #[structopt(long, value_name = "DATA")]
    data: Option<String>,

    /// Absoluate path of the data file to upload.
    #[structopt(long, value_name = "PATH", parse(from_os_str), conflicts_with = "data")]
    path: Option<PathBuf>,
}

impl SharedParams {
    pub fn read_data(&self) -> Result<Vec<u8>> {
        if let Some(ref data) = self.data {
            Ok(data.as_bytes().to_vec())
        } else if let Some(ref path) = self.path {
            std::fs::read(path).map_err(Into::into)
        } else {
            Err(anyhow!(
                "--data or --path is required, please rerun the command with `--help`."
            ))
        }
    }
}

/// Permastore
#[derive(Debug, StructOpt)]
pub enum Permastore {
    /// Submit the `store` extrinsic only.
    Store {
        #[structopt(flatten)]
        shared: SharedParams,
    },
    /// Submit the transction data only.
    Submit {
        #[structopt(flatten)]
        shared: SharedParams,
    },
    /// Submit the `store` extrinsic and the transaction data.
    StoreWithData {
        #[structopt(flatten)]
        shared: SharedParams,
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

        match self {
            Self::Store { shared } => {
                let raw_data = shared.read_data()?;
                store(&client, &signer, raw_data).await?;
            }
            Self::Submit { shared } => {
                let raw_data = shared.read_data()?;
                let permastore_rpc = PermastoreRpc::new(client.rpc_client());
                let ret = permastore_rpc.submit(raw_data.into()).await?;
                println!("Submitted result: {:?}", ret);
            }
            Self::StoreWithData { shared } => {
                let data = shared.read_data()?;

                let chunks = data
                    .chunks(CHUNK_SIZE as usize)
                    .map(|c| BlakeTwo256::hash(c).encode())
                    .collect();

                let chunk_root = BlakeTwo256::ordered_trie_root(chunks);
                let data_size = data.len() as u32;
                println!("data size in bytes: {:?}", data_size);
                println!("        chunk root: {:?}", chunk_root);

                let store_call = crate::pallets::permastore::StoreCall::new(data_size, chunk_root);
                let uxt = client.create_signed(store_call, &signer).await?;

                let permastore_rpc = PermastoreRpc::new(client.rpc_client());
                let ret = permastore_rpc
                    .submit_extrinsic(uxt.encode().into(), data.into())
                    .await?;
                println!("  Submitted result: {:?}", ret);
            }
        }

        Ok(())
    }
}
