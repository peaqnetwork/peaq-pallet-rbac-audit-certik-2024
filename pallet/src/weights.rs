
//! Autogenerated weights for `peaq_pallet_rbac`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `NeutrinoksDell`, CPU: `13th Gen Intel(R) Core(TM) i7-13700H`
//! EXECUTION: Some(Native), WASM-EXECUTION: Compiled, CHAIN: Some("dev-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/peaq-node
// benchmark
// pallet
// --chain=dev-local
// --execution=native
// --wasm-execution=compiled
// --pallet=peaq-pallet-rbac
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `peaq_pallet_rbac`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac RoleStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac RoleStore (max_values: None, max_size: None, mode: Measured)
	fn add_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 25_370_000 picoseconds.
		Weight::from_parts(25_732_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac RoleStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac RoleStore (max_values: None, max_size: None, mode: Measured)
	fn update_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 38_314_000 picoseconds.
		Weight::from_parts(39_387_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac RoleStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac RoleStore (max_values: None, max_size: None, mode: Measured)
	fn disable_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 39_744_000 picoseconds.
		Weight::from_parts(40_437_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191`
		//  Estimated: `3656`
		// Minimum execution time: 20_793_000 picoseconds.
		Weight::from_parts(22_021_000, 0)
			.saturating_add(Weight::from_parts(0, 3656))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac RoleStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac RoleStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_roles() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `232`
		//  Estimated: `3697`
		// Minimum execution time: 20_038_000 picoseconds.
		Weight::from_parts(20_410_000, 0)
			.saturating_add(Weight::from_parts(0, 3697))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac Role2UserStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac Role2UserStore (max_values: None, max_size: None, mode: Measured)
	fn assign_role_to_user() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3646`
		// Minimum execution time: 24_055_000 picoseconds.
		Weight::from_parts(24_463_000, 0)
			.saturating_add(Weight::from_parts(0, 3646))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac Role2UserStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac Role2UserStore (max_values: None, max_size: None, mode: Measured)
	fn unassign_role_to_user() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `3716`
		// Minimum execution time: 24_151_000 picoseconds.
		Weight::from_parts(24_448_000, 0)
			.saturating_add(Weight::from_parts(0, 3716))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:2 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac Role2GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac Role2GroupStore (max_values: None, max_size: None, mode: Measured)
	fn assign_role_to_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `304`
		//  Estimated: `6244`
		// Minimum execution time: 27_972_000 picoseconds.
		Weight::from_parts(29_753_000, 0)
			.saturating_add(Weight::from_parts(0, 6244))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac Role2GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac Role2GroupStore (max_values: None, max_size: None, mode: Measured)
	fn unassign_role_to_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `284`
		//  Estimated: `3749`
		// Minimum execution time: 25_932_000 picoseconds.
		Weight::from_parts(27_224_000, 0)
			.saturating_add(Weight::from_parts(0, 3749))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac Role2UserStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Role2UserStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_user_roles() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `3716`
		// Minimum execution time: 21_574_000 picoseconds.
		Weight::from_parts(21_960_000, 0)
			.saturating_add(Weight::from_parts(0, 3716))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac PermissionStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac PermissionStore (max_values: None, max_size: None, mode: Measured)
	fn add_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 25_332_000 picoseconds.
		Weight::from_parts(26_098_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac PermissionStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac PermissionStore (max_values: None, max_size: None, mode: Measured)
	fn update_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3744`
		// Minimum execution time: 40_613_000 picoseconds.
		Weight::from_parts(44_704_000, 0)
			.saturating_add(Weight::from_parts(0, 3744))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac PermissionStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac PermissionStore (max_values: None, max_size: None, mode: Measured)
	fn disable_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3744`
		// Minimum execution time: 40_220_000 picoseconds.
		Weight::from_parts(41_348_000, 0)
			.saturating_add(Weight::from_parts(0, 3744))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_permission() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `196`
		//  Estimated: `3661`
		// Minimum execution time: 21_670_000 picoseconds.
		Weight::from_parts(22_230_000, 0)
			.saturating_add(Weight::from_parts(0, 3661))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac PermissionStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac PermissionStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_permissions() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `3707`
		// Minimum execution time: 21_193_000 picoseconds.
		Weight::from_parts(21_509_000, 0)
			.saturating_add(Weight::from_parts(0, 3707))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:2 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac Permission2RoleStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac Permission2RoleStore (max_values: None, max_size: None, mode: Measured)
	fn assign_permission_to_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `304`
		//  Estimated: `6244`
		// Minimum execution time: 27_785_000 picoseconds.
		Weight::from_parts(28_486_000, 0)
			.saturating_add(Weight::from_parts(0, 6244))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac Permission2RoleStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac Permission2RoleStore (max_values: None, max_size: None, mode: Measured)
	fn unassign_permission_to_role() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `284`
		//  Estimated: `3749`
		// Minimum execution time: 24_329_000 picoseconds.
		Weight::from_parts(26_495_000, 0)
			.saturating_add(Weight::from_parts(0, 3749))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac Permission2RoleStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Permission2RoleStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_role_permissions() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `284`
		//  Estimated: `3749`
		// Minimum execution time: 22_084_000 picoseconds.
		Weight::from_parts(24_185_000, 0)
			.saturating_add(Weight::from_parts(0, 3749))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac GroupStore (max_values: None, max_size: None, mode: Measured)
	fn add_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 25_672_000 picoseconds.
		Weight::from_parts(28_467_000, 0)
			.saturating_add(Weight::from_parts(0, 3507))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac GroupStore (max_values: None, max_size: None, mode: Measured)
	fn update_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 38_607_000 picoseconds.
		Weight::from_parts(40_654_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac GroupStore (max_values: None, max_size: None, mode: Measured)
	fn disable_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3734`
		// Minimum execution time: 37_844_000 picoseconds.
		Weight::from_parts(38_574_000, 0)
			.saturating_add(Weight::from_parts(0, 3734))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `191`
		//  Estimated: `3656`
		// Minimum execution time: 20_541_000 picoseconds.
		Weight::from_parts(20_877_000, 0)
			.saturating_add(Weight::from_parts(0, 3656))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac GroupStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac GroupStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_groups() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `232`
		//  Estimated: `3697`
		// Minimum execution time: 20_742_000 picoseconds.
		Weight::from_parts(21_246_000, 0)
			.saturating_add(Weight::from_parts(0, 3697))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac User2GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac User2GroupStore (max_values: None, max_size: None, mode: Measured)
	fn assign_user_to_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3646`
		// Minimum execution time: 25_279_000 picoseconds.
		Weight::from_parts(26_027_000, 0)
			.saturating_add(Weight::from_parts(0, 3646))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac User2GroupStore (r:1 w:1)
	/// Proof Skipped: PeaqRbac User2GroupStore (max_values: None, max_size: None, mode: Measured)
	fn unassign_user_to_group() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `3716`
		// Minimum execution time: 25_095_000 picoseconds.
		Weight::from_parts(25_682_000, 0)
			.saturating_add(Weight::from_parts(0, 3716))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: PeaqRbac User2GroupStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac User2GroupStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_user_groups() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `251`
		//  Estimated: `3716`
		// Minimum execution time: 22_532_000 picoseconds.
		Weight::from_parts(22_863_000, 0)
			.saturating_add(Weight::from_parts(0, 3716))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	/// Storage: PeaqRbac Role2UserStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Role2UserStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac User2GroupStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac User2GroupStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac Role2GroupStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Role2GroupStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac Permission2RoleStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Permission2RoleStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_user_permissions() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `742`
		//  Estimated: `4207`
		// Minimum execution time: 45_668_000 picoseconds.
		Weight::from_parts(46_782_000, 0)
			.saturating_add(Weight::from_parts(0, 4207))
			.saturating_add(T::DbWeight::get().reads(5))
	}
	/// Storage: PeaqRbac Role2GroupStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Role2GroupStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac Permission2RoleStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Permission2RoleStore (max_values: None, max_size: None, mode: Measured)
	/// Storage: PeaqRbac KeysLookUpStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac KeysLookUpStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_group_permissions() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `604`
		//  Estimated: `4069`
		// Minimum execution time: 37_319_000 picoseconds.
		Weight::from_parts(38_150_000, 0)
			.saturating_add(Weight::from_parts(0, 4069))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	/// Storage: PeaqRbac Role2GroupStore (r:1 w:0)
	/// Proof Skipped: PeaqRbac Role2GroupStore (max_values: None, max_size: None, mode: Measured)
	fn fetch_group_roles() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `284`
		//  Estimated: `3749`
		// Minimum execution time: 22_514_000 picoseconds.
		Weight::from_parts(22_988_000, 0)
			.saturating_add(Weight::from_parts(0, 3749))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}
