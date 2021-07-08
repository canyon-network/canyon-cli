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
    /// Send extrinsic for storing data.
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
}

/// Send `permastore::call` extrinsic.
async fn store(client: &CanyonClient, signer: &CanyonSigner, data: Vec<u8>) -> Result<()> {
    let chunks = data
        .chunks(CHUNK_SIZE as usize)
        .map(|c| BlakeTwo256::hash(c).encode())
        .collect();

    let chunk_root = BlakeTwo256::ordered_trie_root(chunks);
    println!("chunk root: {:?}", chunk_root);

    let data_size = data.len() as u32;
    println!("data size: {:?}", data_size);

    let result = client.store(signer, data_size, chunk_root, data).await?;
    println!("Store result: {:?}", result);

    Ok(())
}

/// Submit the transaction data asynchonously.
async fn submit(rpc_client: &RpcClient, value: Bytes) -> Result<H256> {
    let params = &[to_json_value(value)?];
    let data = rpc_client.request("permastore_submit", params).await?;
    Ok(data)
}

impl Permastore {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Store { data, path } => {
                let raw_data = if let Some(data) = data {
                    data.as_bytes().to_vec()
                } else if let Some(path) = path {
                    std::fs::read(path)?
                } else {
                    return Err(anyhow!("--data or --path is required for store command"));
                };

                store(&client, &signer, raw_data).await?;
            }
            Self::Submit { data, path } => {
                let raw_data = if let Some(data) = data {
                    data.as_bytes().to_vec()
                } else if let Some(path) = path {
                    std::fs::read(path)?
                } else {
                    return Err(anyhow!("--data or --path is required for store command"));
                };

                let rpc_client = client.rpc_client();
                let ret = submit(rpc_client, raw_data.into()).await?;
                println!("Submitted result: {:?}", ret);
            }
        }

        Ok(())
    }
}
