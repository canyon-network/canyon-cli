use std::marker::PhantomData;

use codec::Encode;
use subxt::{balances::Balances, module, system::System, Call};

#[module]
pub trait Permastore: Balances + System {}

#[derive(Clone, Debug, PartialEq, Encode, Call)]
pub struct StoreCall<T: Permastore> {
    /// Byte size of `data`.
    pub data_size: u32,
    /// Merkle root of the transaction data in chunks.
    pub chunk_root: T::Hash,
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}
