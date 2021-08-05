pub mod permastore {
    use std::marker::PhantomData;
    use codec::Encode;
    use subxt::{balances::Balances, module, system::System, Call};
    pub trait Permastore: Balances + System {}
    const MODULE: &str = "Permastore";
    /// `EventTypeRegistry` extension trait.
    pub trait PermastoreEventTypeRegistry {
        /// Registers this modules types.
        fn with_permastore(&mut self);
    }
    impl<T: Permastore + subxt::Runtime> PermastoreEventTypeRegistry for subxt::EventTypeRegistry<T> {
        fn with_permastore(&mut self) {}
    }
    pub struct StoreCall<T: Permastore> {
        /// Byte size of `data`.
        pub data_size: u32,
        /// Merkle root of the transaction data in chunks.
        pub chunk_root: T::Hash,
        /// Runtime marker.
        pub _runtime: PhantomData<T>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Permastore> ::core::clone::Clone for StoreCall<T>
    where
        T::Hash: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> StoreCall<T> {
            match *self {
                StoreCall {
                    data_size: ref __self_0_0,
                    chunk_root: ref __self_0_1,
                    _runtime: ref __self_0_2,
                } => StoreCall {
                    data_size: ::core::clone::Clone::clone(&(*__self_0_0)),
                    chunk_root: ::core::clone::Clone::clone(&(*__self_0_1)),
                    _runtime: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Permastore> ::core::fmt::Debug for StoreCall<T>
    where
        T::Hash: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                StoreCall {
                    data_size: ref __self_0_0,
                    chunk_root: ref __self_0_1,
                    _runtime: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "StoreCall");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "data_size",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "chunk_root",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "_runtime",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl<T: Permastore> ::core::marker::StructuralPartialEq for StoreCall<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq + Permastore> ::core::cmp::PartialEq for StoreCall<T>
    where
        T::Hash: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &StoreCall<T>) -> bool {
            match *other {
                StoreCall {
                    data_size: ref __self_1_0,
                    chunk_root: ref __self_1_1,
                    _runtime: ref __self_1_2,
                } => match *self {
                    StoreCall {
                        data_size: ref __self_0_0,
                        chunk_root: ref __self_0_1,
                        _runtime: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &StoreCall<T>) -> bool {
            match *other {
                StoreCall {
                    data_size: ref __self_1_0,
                    chunk_root: ref __self_1_1,
                    _runtime: ref __self_1_2,
                } => match *self {
                    StoreCall {
                        data_size: ref __self_0_0,
                        chunk_root: ref __self_0_1,
                        _runtime: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    const _: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate codec as _parity_scale_codec;
        impl<T: Permastore> _parity_scale_codec::Encode for StoreCall<T>
        where
            T::Hash: _parity_scale_codec::Encode,
            T::Hash: _parity_scale_codec::Encode,
            PhantomData<T>: _parity_scale_codec::Encode,
            PhantomData<T>: _parity_scale_codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: _parity_scale_codec::Output + ?Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                _parity_scale_codec::Encode::encode_to(&self.data_size, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self.chunk_root, __codec_dest_edqy);
                _parity_scale_codec::Encode::encode_to(&self._runtime, __codec_dest_edqy);
            }
        }
        impl<T: Permastore> _parity_scale_codec::EncodeLike for StoreCall<T>
        where
            T::Hash: _parity_scale_codec::Encode,
            T::Hash: _parity_scale_codec::Encode,
            PhantomData<T>: _parity_scale_codec::Encode,
            PhantomData<T>: _parity_scale_codec::Encode,
        {
        }
    };
    impl<T: Permastore> subxt::Call<T> for StoreCall<T> {
        const MODULE: &'static str = MODULE;
        const FUNCTION: &'static str = "store";
    }
    /// Call extension trait.
    pub trait StoreCallExt<T: subxt::Runtime + Permastore> {
        /// Create and submit an extrinsic.
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn store<'a, 'async_trait>(
            &'a self,
            signer: &'a (dyn subxt::Signer<T> + Send + Sync),
            data_size: u32,
            chunk_root: T::Hash,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<T::Hash, subxt::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'a: 'async_trait,
            Self: 'async_trait;
        /// Create, submit and watch an extrinsic.
        #[must_use]
        #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
        fn store_and_watch<'a, 'async_trait>(
            &'a self,
            signer: &'a (dyn subxt::Signer<T> + Send + Sync),
            data_size: u32,
            chunk_root: T::Hash,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<subxt::ExtrinsicSuccess<T>, subxt::Error>,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'a: 'async_trait,
            Self: 'async_trait;
    }
    impl<T: subxt::Runtime + Permastore> StoreCallExt<T> for subxt::Client<T>
    where
        <<T::Extra as subxt::SignedExtra<T>>::Extra as subxt::SignedExtension>::AdditionalSigned:
            Send + Sync,
    {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn store<'a, 'async_trait>(
            &'a self,
            signer: &'a (dyn subxt::Signer<T> + Send + Sync),
            data_size: u32,
            chunk_root: T::Hash,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = Result<T::Hash, subxt::Error>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'a: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<T::Hash, subxt::Error>>
                {
                    return __ret;
                }
                let __self = self;
                let signer = signer;
                let data_size = data_size;
                let chunk_root = chunk_root;
                let __ret: Result<T::Hash, subxt::Error> = {
                    let _runtime = core::marker::PhantomData::<T>;
                    __self
                        .submit(
                            StoreCall {
                                data_size,
                                chunk_root,
                                _runtime,
                            },
                            signer,
                        )
                        .await
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn store_and_watch<'a, 'async_trait>(
            &'a self,
            signer: &'a (dyn subxt::Signer<T> + Send + Sync),
            data_size: u32,
            chunk_root: T::Hash,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = Result<subxt::ExtrinsicSuccess<T>, subxt::Error>,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'a: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<Result<subxt::ExtrinsicSuccess<T>, subxt::Error>>
                {
                    return __ret;
                }
                let __self = self;
                let signer = signer;
                let data_size = data_size;
                let chunk_root = chunk_root;
                let __ret: Result<subxt::ExtrinsicSuccess<T>, subxt::Error> = {
                    let _runtime = core::marker::PhantomData::<T>;
                    __self
                        .watch(
                            StoreCall {
                                data_size,
                                chunk_root,
                                _runtime,
                            },
                            signer,
                        )
                        .await
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
}
