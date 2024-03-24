// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-j8vvqcjr-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("coretime-kusama-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=coretime-kusama-dev
// --wasm-execution=compiled
// --pallet=pallet_multisig
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/coretime/coretime-kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_905_000 picoseconds.
		Weight::from_parts(13_544_225, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(596, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `262 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 38_729_000 picoseconds.
		Weight::from_parts(27_942_442, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 648
			.saturating_add(Weight::from_parts(120_340, 0).saturating_mul(s.into()))
			// Standard Error: 6
			.saturating_add(Weight::from_parts(1_578, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `6811`
		// Minimum execution time: 25_936_000 picoseconds.
		Weight::from_parts(16_537_903, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 412
			.saturating_add(Weight::from_parts(105_835, 0).saturating_mul(s.into()))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(1_534, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `385 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 45_291_000 picoseconds.
		Weight::from_parts(31_294_385, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 816
			.saturating_add(Weight::from_parts(152_838, 0).saturating_mul(s.into()))
			// Standard Error: 8
			.saturating_add(Weight::from_parts(1_638, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263 + s * (2 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 26_585_000 picoseconds.
		Weight::from_parts(27_424_168, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 732
			.saturating_add(Weight::from_parts(123_460, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `282`
		//  Estimated: `6811`
		// Minimum execution time: 15_228_000 picoseconds.
		Weight::from_parts(15_568_631, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 441
			.saturating_add(Weight::from_parts(107_463, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `454 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 28_033_000 picoseconds.
		Weight::from_parts(29_228_827, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 748
			.saturating_add(Weight::from_parts(117_495, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}