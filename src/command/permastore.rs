use std::path::PathBuf;

use anyhow::{anyhow, Result};
use sp_core::Encode;
use sp_runtime::traits::{BlakeTwo256, Hash};
use structopt::StructOpt;

use cp_permastore::CHUNK_SIZE;

use crate::{
    runtime::{pallets::permastore::StoreCallExt, CanyonClient, CanyonSigner},
    utils::build_client,
};

/// Permastore
#[derive(Debug, StructOpt)]
pub enum Permastore {
    /// Store the data.
    Store {
        #[structopt(long)]
        data: Option<String>,
        #[structopt(short, long, parse(from_os_str), conflicts_with = "data")]
        path: Option<PathBuf>,
    },
}

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

impl Permastore {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Store { data, path } => {
                if let Some(data) = data {
                    let data = data.as_bytes().to_vec();
                    store(&client, &signer, data).await?;
                } else if let Some(path) = path {
                    let data = std::fs::read(path)?;
                    store(&client, &signer, data).await?;
                } else {
                    return Err(anyhow!("--data or --path is required for store command"));
                }
            }
        }

        Ok(())
    }
}
