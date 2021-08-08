use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;
use subxt::system::{AccountStoreExt, SetCodeWithoutChecksCallExt};

use crate::{
    client::CanyonClient,
    runtime::{
        primitives::{AccountId, BlockNumber},
        CanyonSigner,
    },
    utils::{parse_account, read_code},
};

/// System
#[derive(Debug, StructOpt)]
pub enum System {
    /// Get the account information.
    AccountInfo {
        #[structopt(index = 1, long, parse(try_from_str = parse_account))]
        who: AccountId,
        #[structopt(long)]
        block_number: Option<BlockNumber>,
    },
    /// Set code without checking.
    SetCodeWithoutChecks {
        /// Code path
        #[structopt(index = 1, long, parse(from_os_str))]
        code: PathBuf,
    },
}

impl System {
    pub async fn run(self, url: String, signer: CanyonSigner) -> Result<()> {
        let client = CanyonClient::create(url).await?;

        match self {
            Self::AccountInfo { who, block_number } => {
                let at = client.block_hash(block_number).await?;
                let account_info = client.0.account(&who, at).await?;
                println!("AccountInfo of {:?}: {:#?}", who, account_info);
            }
            Self::SetCodeWithoutChecks { code } => {
                let result = client
                    .0
                    .set_code_without_checks_and_watch(&signer, &read_code(code)?)
                    .await?;
                println!("set_code_without_checks result:{:#?}", result);
            }
        }

        Ok(())
    }
}
