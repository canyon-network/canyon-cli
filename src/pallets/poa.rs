use pallet_poa::DepthInfo;

use codec::Encode;
use subxt::{module, system::System, Store};

#[module]
pub trait Poa: System {}

/// The history depth of the poa module.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct HistoryDepthStore<'a, T: System> {
    #[store(returns = DepthInfo<T::BlockNumber>)]
    /// Account to retrieve the `AccountInfo<T>` for.
    pub account_id: &'a T::AccountId,
}
