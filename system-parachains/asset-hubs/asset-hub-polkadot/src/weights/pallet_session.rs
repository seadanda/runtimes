// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_session`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a3dce7bd4066`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("spec-asset-hub-polkadot.json")`, DB CACHE: 1024

// Executed Command:
// /builds/polkadot-sdk/target/production/polkadot-parachain
// benchmark
// pallet
// --chain=spec-asset-hub-polkadot.json
// --pallet=pallet_session
// --extrinsic=
// --output=/builds/runtimes/system-parachains/asset-hubs/asset-hub-polkadot/src/weights
// --header=/builds/bench/header.txt
// --no-median-slopes
// --no-min-squares

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_session`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_session::WeightInfo for WeightInfo<T> {
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::KeyOwner` (r:1 w:1)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270`
		//  Estimated: `3735`
		// Minimum execution time: 15_655_000 picoseconds.
		Weight::from_parts(16_027_000, 0)
			.saturating_add(Weight::from_parts(0, 3735))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Session::NextKeys` (r:1 w:1)
	/// Proof: `Session::NextKeys` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Session::KeyOwner` (r:0 w:1)
	/// Proof: `Session::KeyOwner` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn purge_keys() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `3707`
		// Minimum execution time: 11_194_000 picoseconds.
		Weight::from_parts(11_687_000, 0)
			.saturating_add(Weight::from_parts(0, 3707))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}