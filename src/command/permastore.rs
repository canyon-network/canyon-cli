use anyhow::Result;
use sp_runtime::traits::Hash;
use structopt::StructOpt;

use crate::{
    runtime::{pallets::permastore::StoreCallExt, CanyonSigner},
    utils::build_client,
};

/// Permastore
#[derive(Debug, StructOpt)]
pub enum Permastore {
    /// Store the data.
    Store {
        #[structopt(index = 1, long)]
        data: String,
    },
}

impl Permastore {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = build_client(url).await?;

        match self {
            Self::Store { data } => {
                const CHUNK_SIZE: usize = 256 * 1024;

                let data = data.as_bytes().to_vec();
                let chunks = data.chunks(CHUNK_SIZE).map(|c| c.to_vec()).collect();
                let chunk_root = sp_runtime::traits::BlakeTwo256::ordered_trie_root(chunks);
                let data_size = data.len() as u32;

                let result = client.store(&signer, data_size, chunk_root, data).await?;
                println!("Store result: {:?}", result);
            }
        }

        Ok(())
    }
}
