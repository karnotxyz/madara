// A few exports that help ease life for downstream crates.
pub use frame_support::traits::{ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness, StorageInfo};
pub use frame_support::weights::constants::{
    BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
};
pub use frame_support::weights::{IdentityFee, Weight};
pub use frame_support::{construct_runtime, parameter_types, StorageValue};
pub use frame_system::Call as SystemCall;
/// Import the Starknet pallet.
pub use pallet_starknet;
pub use pallet_timestamp::Call as TimestampCall;
use sp_core::H256;
use sp_runtime::traits::BlakeTwo256;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
use sp_runtime::{generic, impl_opaque_keys};
pub use sp_runtime::{Perbill, Permill};
use sp_std::prelude::*;

use super::*;
use crate::{Aura, BlockNumber, Grandpa};

// Deoxys block header type
pub type DHeaderT = generic::Header<BlockNumber, BlakeTwo256>;
/// Deoxys block type.
pub type DBlockT = generic::Block<Header, UncheckedExtrinsic>;
/// Deoxys block hash type.
pub type DHashT = H256;
/// Deoxys block identifier type.
pub type BlockId = generic::BlockId<DBlockT>;

impl_opaque_keys! {
    pub struct SessionKeys {
        pub aura: Aura,
        pub grandpa: Grandpa,
    }
}
