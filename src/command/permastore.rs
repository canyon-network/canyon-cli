use std::path::PathBuf;

use anyhow::{anyhow, Result};
use jsonrpsee_types::to_json_value;
use sp_core::{hashing::twox_128, Bytes, Encode, H256};
use sp_runtime::traits::{BlakeTwo256, Hash as HashT};
use structopt::StructOpt;
use subxt::RpcClient;

use cp_permastore::CHUNK_SIZE;

use crate::client::CanyonClient;
use crate::runtime::{primitives::Hash, CanyonSigner};

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
        /// Prepare and display the data info but not send it.
        #[structopt(long)]
        dry_run: bool,
    },
    /// Submit the `store` extrinsic and the transaction data.
    StoreWithData {
        #[structopt(flatten)]
        shared: SharedParams,
        /// Prepare and display the data info but not send it.
        #[structopt(long)]
        dry_run: bool,
    },
    /// Remove data.
    Remove {
        /// Chunk root of that data you want to delete.
        #[structopt(index = 1, long)]
        chunk_root: String,
    },
}

pub fn final_storage_prefix(pallet_prefix: &str, storage_prefix: &str) -> Vec<u8> {
    let mut final_prefix = twox_128(pallet_prefix.as_bytes()).to_vec();
    final_prefix.extend_from_slice(&twox_128(storage_prefix.as_bytes()));
    final_prefix
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

    ///
    async fn remove_data(&self, chunk_root: Hash) -> Result<bool> {
        let params = &[to_json_value(chunk_root)?];
        let data = self.rpc.request("permastore_removeData", params).await?;
        Ok(data)
    }
}

impl Permastore {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = CanyonClient::create(url).await?;

        let process_data = |data: &[u8]| {
            let chunks = data
                .chunks(CHUNK_SIZE as usize)
                .map(|c| BlakeTwo256::hash(c).encode())
                .collect();

            let chunk_root = BlakeTwo256::ordered_trie_root(chunks);
            let data_size = data.len() as u32;
            (chunk_root, data_size)
        };

        match self {
            Self::Store { shared } => {
                let raw_data = shared.read_data()?;
                client.store(&signer, raw_data).await?;
            }
            Self::Submit { shared, dry_run } => {
                let raw_data = shared.read_data()?;
                if dry_run {
                    let (chunk_root, data_size) = process_data(&raw_data);
                    println!("data size in bytes: {:?}", data_size);
                    println!("        chunk root: {:?}", chunk_root);
                } else {
                    let permastore_rpc = PermastoreRpc::new(client.rpc_client());
                    let ret = permastore_rpc.submit(raw_data.into()).await?;
                    println!("Submitted result: {:?}", ret);
                }
            }
            Self::StoreWithData { shared, dry_run } => {
                let raw_data = shared.read_data()?;

                let (chunk_root, data_size) = process_data(&raw_data);
                println!("data size in bytes: {:?}", data_size);
                println!("        chunk root: {:?}", chunk_root);

                if !dry_run {
                    let store_call =
                        crate::pallets::permastore::StoreCall::new(data_size, chunk_root);
                    let uxt = client.0.create_signed(store_call, &signer).await?;

                    let permastore_rpc = PermastoreRpc::new(client.rpc_client());
                    let ret = permastore_rpc
                        .submit_extrinsic(uxt.encode().into(), raw_data.into())
                        .await?;
                    println!("  Submitted result: {:?}", ret);
                }
            }
            Self::Remove { chunk_root } => {
                let mut bytes = [0u8; 32];
                hex::decode_to_slice(
                    if let Some(s) = chunk_root.strip_prefix("0x") {
                        s
                    } else {
                        &chunk_root
                    },
                    &mut bytes as &mut [u8],
                )?;

                let permastore_rpc = PermastoreRpc::new(client.rpc_client());

                let ret = permastore_rpc.remove_data(bytes.into()).await?;
                println!("  Submitted result: {:?}", ret);
            }
        }

        Ok(())
    }
}
