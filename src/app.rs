use anyhow::{anyhow, Result};
use sp_core::Pair;
use sp_keyring::AccountKeyring;
use structopt::{
    clap::{arg_enum, AppSettings::ColoredHelp},
    StructOpt,
};
use subxt::PairSigner;

use crate::runtime::CanyonSigner;

#[derive(StructOpt, Debug)]
pub enum Command {
    Balances(crate::command::balances::Balances),
    System(crate::command::system::System),
    Permastore(crate::command::permastore::Permastore),
    InspectKey,
}

arg_enum! {
  #[derive(Clone, Debug)]
  pub enum BuiltinAccounts {
      Alice,
      Bob,
      Charlie,
      Dave,
      Eve,
      Ferdie,
      One,
      Two,
  }
}

impl Into<AccountKeyring> for BuiltinAccounts {
    fn into(self) -> AccountKeyring {
        match self {
            Self::Alice => AccountKeyring::Alice,
            Self::Bob => AccountKeyring::Bob,
            Self::Charlie => AccountKeyring::Charlie,
            Self::Dave => AccountKeyring::Dave,
            Self::Eve => AccountKeyring::Eve,
            Self::Ferdie => AccountKeyring::Ferdie,
            Self::One => AccountKeyring::One,
            Self::Two => AccountKeyring::Two,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "canyon-cli",
    author,
    about,
    no_version,
    global_setting(ColoredHelp)
)]
pub struct App {
    /// Builtin test accounts.
    #[structopt(long, possible_values = &BuiltinAccounts::variants(), case_insensitive = true)]
    pub signer: Option<BuiltinAccounts>,

    /// A Key URI used as a signer.
    ///
    /// Maybe a secret seed, secret URI(with derivation paths and password), SS58 or public URI.
    /// You can also use an environment variable URI=[URI] for this purpose.
    #[structopt(long)]
    pub uri: Option<String>,

    /// The websocket url of Canyon node.
    #[structopt(long, default_value = "ws://127.0.0.1:9944")]
    pub url: String,

    /// Ss58 Address version of the network.
    #[structopt(long, default_value = "42")]
    pub ss58_prefix: sp_core::crypto::Ss58AddressFormat,

    #[structopt(subcommand)]
    pub command: Command,
}

fn as_sr25519_signer(uri: &str) -> Result<CanyonSigner> {
    sp_core::sr25519::Pair::from_phrase(&uri, None)
        .map(|(pair, _seed)| PairSigner::new(pair))
        .map_err(|err| anyhow!("Failed to generate sr25519 Pair from uri: {:?}", err))
}

impl App {
    pub fn init() -> Self {
        App::from_args()
    }

    pub async fn run(self) -> Result<()> {
        sp_core::crypto::set_default_ss58_version(self.ss58_prefix);

        let signer = if let Some(ref uri) = self.get_uri() {
            as_sr25519_signer(uri)?
        } else {
            self.builtin_signer()
        };

        match self.command {
            Command::Balances(balances) => balances.run(self.url, signer).await?,
            Command::System(system) => system.run(self.url, signer).await?,
            Command::Permastore(permastore) => permastore.run(self.url, signer).await?,
            Command::InspectKey => {
                if let Some(ref uri) = self.get_uri() {
                    sc_cli::utils::print_from_uri::<sp_core::sr25519::Pair>(
                        uri,
                        None,
                        Some(self.ss58_prefix),
                        sc_cli::OutputType::Text,
                    );
                }
            }
        }

        Ok(())
    }

    fn get_uri(&self) -> Option<String> {
        if let Some(ref uri) = self.uri {
            Some(uri.into())
        } else if let Ok(ref uri) = std::env::var("URI") {
            Some(uri.into())
        } else {
            None
        }
    }

    fn builtin_signer(&self) -> CanyonSigner {
        let signer = self.signer.clone().unwrap_or(BuiltinAccounts::Alice);
        let signer: AccountKeyring = signer.into();
        PairSigner::new(signer.pair())
    }
}
