use std::marker::PhantomData;

use codec::Encode;
use subxt::{balances::Balances, module, system::System, Call, Store};

#[module]
pub trait Permastore: Balances + System {}

/// Store the data onto the network.
#[derive(Clone, Debug, PartialEq, Encode, Call)]
pub struct StoreCall<T: Permastore> {
    /// Byte size of `data`.
    pub data_size: u32,
    /// Merkle root of the transaction data in chunks.
    pub chunk_root: T::Hash,
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

impl<T: Permastore> StoreCall<T> {
    pub fn new(data_size: u32, chunk_root: T::Hash) -> Self {
        Self {
            data_size,
            chunk_root,
            _runtime: PhantomData::<T>,
        }
    }
}

/// The size of entire weave.
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct WeaveSizeStore<T: Permastore> {
    #[store(returns = u64)]
    pub _runtime: PhantomData<T>,
}
