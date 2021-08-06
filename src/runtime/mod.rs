pub mod primitives;

use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{generic::Header, impl_opaque_keys, OpaqueExtrinsic};
use subxt::{
    balances::{AccountData, Balances, BalancesEventTypeRegistry},
    extrinsic::DefaultExtra,
    register_default_type_sizes,
    session::{Session, SessionEventTypeRegistry},
    sudo::{Sudo, SudoEventTypeRegistry},
    system::{System, SystemEventTypeRegistry},
    Client, EventTypeRegistry, PairSigner, Runtime,
};

use self::primitives::*;

/// Concrete type definitions for Canyon.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CanyonRuntime;

impl Runtime for CanyonRuntime {
    type Signature = Signature;
    type Extra = DefaultExtra<Self>;
    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_system();
        event_type_registry.with_balances();
        event_type_registry.with_session();
        // event_type_registry.with_staking();
        event_type_registry.with_sudo();
        register_default_type_sizes(event_type_registry);
    }
}

impl System for CanyonRuntime {
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = Hashing;
    type AccountId = AccountId;
    type Address = Address;
    type Header = Header<Self::BlockNumber, Self::Hashing>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Sudo for CanyonRuntime {}

impl Balances for CanyonRuntime {
    type Balance = Balance;
}

/// BABE marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Babe;
impl sp_runtime::BoundToRuntimeAppPublic for Babe {
    type Public = BabeId;
}

/// GRANDPA marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Grandpa;
impl sp_runtime::BoundToRuntimeAppPublic for Grandpa {
    type Public = GrandpaId;
}

/// ImOnline marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ImOnline;
impl sp_runtime::BoundToRuntimeAppPublic for ImOnline {
    type Public = ImOnlineId;
}

/// Authority discovery marker struct
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AuthorityDiscovery;
impl sp_runtime::BoundToRuntimeAppPublic for AuthorityDiscovery {
    type Public = AuthorityDiscoveryId;
}

impl_opaque_keys! {
    /// Substrate base runtime keys
    pub struct BasicSessionKeys {
        /// BABE session key
        pub babe: Babe,
        /// GRANDPA session key
        pub grandpa: Grandpa,
        /// ImOnline session key
        pub im_online: ImOnline,
        /// AuthorityDiscovery session key
        pub authority_discovery: AuthorityDiscovery,
    }
}

impl Session for CanyonRuntime {
    type ValidatorId = <Self as System>::AccountId;
    type Keys = BasicSessionKeys;
}

impl crate::pallets::permastore::Permastore for CanyonRuntime {}

/// Canyon `Client` for Canyon runtime.
pub type CanyonClient = Client<CanyonRuntime>;

/// Canyon `Pair` for Canyon runtime.
pub type CanyonPair = sr25519::Pair;

/// Canyon `PairSigner` for Canyon runtime.
pub type CanyonSigner = PairSigner<CanyonRuntime, CanyonPair>;
