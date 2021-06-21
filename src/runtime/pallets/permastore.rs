use std::marker::PhantomData;

use codec::Encode;
use subxt::{balances::Balances, module, system::System};

#[module]
pub trait Permastore: Balances + System {}

#[derive(Clone, Debug, PartialEq, Encode)]
pub struct StoreCall<T: Permastore> {
    /// Raw bytes of transaction data.
    pub data: Vec<u8>,
    /// Byte size of `data`.
    pub data_size: u32,
    /// Merkle root of `data` in chunks.
    pub chunk_root: T::Hash,
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
}

// Impl Call by hand.
impl<T: Permastore> subxt::Call<T> for StoreCall<T> {
    const MODULE: &'static str = MODULE;
    const FUNCTION: &'static str = "store";
}

/// Call extension trait.
#[async_trait::async_trait]
pub trait StoreCallExt<T: subxt::Runtime + Balances> {
    /// Create and submit an extrinsic.
    async fn store<'a>(
        &'a self,
        signer: &'a (dyn subxt::Signer<T> + Send + Sync),
        data_size: u32,
        chunk_root: T::Hash,
        data: Vec<u8>,
    ) -> Result<T::Hash, subxt::Error>;

    /// Create, submit and watch an extrinsic.
    async fn store_and_watch<'a>(
        &'a self,
        signer: &'a (dyn subxt::Signer<T> + Send + Sync),
        data_size: u32,
        chunk_root: T::Hash,
        data: Vec<u8>,
    ) -> Result<subxt::ExtrinsicSuccess<T>, subxt::Error>;
}

#[async_trait::async_trait]
impl<T: subxt::Runtime + Permastore> StoreCallExt<T> for subxt::Client<T>
where
    <<T::Extra as subxt::SignedExtra<T>>::Extra as subxt::SignedExtension>::AdditionalSigned:
        Send + Sync,
{
    async fn store<'a>(
        &'a self,
        signer: &'a (dyn subxt::Signer<T> + Send + Sync),
        data_size: u32,
        chunk_root: T::Hash,
        data: Vec<u8>,
    ) -> Result<T::Hash, subxt::Error> {
        let _runtime = core::marker::PhantomData::<T>;
        self.submit_with_data(
            StoreCall {
                data: data.clone(),
                data_size,
                chunk_root,
                _runtime,
            },
            signer,
            data,
        )
        .await
    }

    async fn store_and_watch<'a>(
        &'a self,
        signer: &'a (dyn subxt::Signer<T> + Send + Sync),
        data_size: u32,
        chunk_root: T::Hash,
        data: Vec<u8>,
    ) -> Result<subxt::ExtrinsicSuccess<T>, subxt::Error> {
        let _runtime = core::marker::PhantomData::<T>;
        self.watch(
            StoreCall {
                data,
                data_size,
                chunk_root,
                _runtime,
            },
            signer,
        )
        .await
    }
}
