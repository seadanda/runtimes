// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use core::time;

use crate::coretime::{BrokerPalletId, CoretimeBurnPotId, PriceAdapter};
use emulated_integration_tests_common::accounts::ALICE;
use frame_support::{
	assert_ok,
	traits::{
		fungible::{Inspect, Mutate},
		OnInitialize,
	},
	StorageValue,
};
use pallet_broker::{AdaptPrice, ConfigRecordOf};
use parachains_runtimes_test_utils::ExtBuilder;
use sp_runtime::{traits::One, FixedU64};

use crate::{
	xcm_config::{BrokerPalletLocation, RelayTreasuryPalletAccount, TreasuryAccount},
	*,
};

#[test]
fn adapt_price_no_panic() {
	for limit in 0..10 {
		for target in 1..10 {
			for sold in 0..=limit {
				let price = PriceAdapter::adapt_price(sold, target, limit);

				if sold > target {
					assert!(price > FixedU64::one());
				} else {
					assert!(price <= FixedU64::one());
				}
			}
		}
	}
}

#[test]
fn adapt_price_bound_check() {
	// Using assumptions from pallet implementation i.e. `limit >= sold`.
	// Check extremes
	let limit = 10;
	let target = 5;

	// Maximally sold: `sold == limit`
	assert_eq!(PriceAdapter::adapt_price(limit, target, limit), FixedU64::from_float(1.2));
	// Ideally sold: `sold == target`
	assert_eq!(PriceAdapter::adapt_price(target, target, limit), FixedU64::one());
	// Minimally sold: `sold == 0`
	assert_eq!(PriceAdapter::adapt_price(0, target, limit), FixedU64::from_float(0.5));
	// Optimistic target: `target == limit`
	assert_eq!(PriceAdapter::adapt_price(limit, limit, limit), FixedU64::one());
	// Pessimistic target: `target == 0`
	assert_eq!(PriceAdapter::adapt_price(limit, 0, limit), FixedU64::from_float(1.2));
}

#[test]
fn leadin_price_bound_check() {
	// Using assumptions from pallet implementation i.e. `when` in range [0, 1].
	// Linear, therefore we need three points to fully test it

	// Start of leadin: 5x
	assert_eq!(PriceAdapter::leadin_factor_at(FixedU64::from(0)), FixedU64::from(5));
	// Midway through leadin: 3x
	assert_eq!(PriceAdapter::leadin_factor_at(FixedU64::from_float(0.5)), FixedU64::from(3));
	// End of leadin: 1x
	assert_eq!(PriceAdapter::leadin_factor_at(FixedU64::one()), FixedU64::one());
}

fn advance_to(b: BlockNumber) {
	while System::block_number() < b {
		let block_number = System::block_number() + 1;
		System::set_block_number(block_number);
		Broker::on_initialize(block_number);
	}
}

#[test]
fn bulk_revenue_is_burnt() {
	const ALICE: &str = "Alice";
	ExtBuilder::<Runtime>::default()
		.with_collators(vec![AccountId::from(ALICE)])
		.with_session_keys(vec![(
			AccountId::from(ALICE),
			AccountId::from(ALICE),
			SessionKeys { aura: AuraId::from(sp_core::ed25519::Public::from_raw(ALICE)) },
		)])
		.build()
		.execute_with(|| {
			let timeslice_period = <Runtime as pallet_broker::Config>::TimeslicePeriod::get();
			// Configure broker and start sales
			let config = ConfigRecordOf::<Runtime> {
				advance_notice: 1,
				interlude_length: 1,
				leadin_length: 1,
				region_length: 1,
				ideal_bulk_proportion: Perbill::from_percent(100),
				limit_cores_offered: None,
				renewal_bump: Perbill::from_percent(3),
				contribution_timeout: 1,
			};
			assert_ok!(Broker::configure(coretime_root_origin.clone(), config.clone()));
			assert_ok!(Broker::start_sales(coretime_root_origin.clone(), 100, 1));

			let sale_start = SaleInfo::<Runtime>::get().unwrap().sale_start;
			advance_to(sale_start + timeslice_period * config.interlude_length);

			// Check and set initial balances.
			let broker_account = Accountd::from(BrokerPalletId);
			let coretime_burn_pot = Accountd::from(CoretimeBurnPotId);
			let treasury_account = RelayTreasuryPalletAccount::get();
			assert_ok!(Balances::mint_into(&AccountId::from(ALICE), 1000));
			assert_eq!(treasury_account.balance(), 0);
			assert_eq!(broker_account.balance(), 0);

			assert_ok!(Broker::purchase(&AccountId::from(ALICE), 500));
			// Alice decreases
			assert_eq!(Balances::balance(ALICE), 500);
			// Treasury balance does not increase
			assert_eq!(Balances::balance(treasury_account), 0);
			// Broker pallet account does not increase
			assert_eq!(Balances::balance(broker_account), 0);
			// Coretime burn pot gets the funds
			assert_eq!(Balances::balance(coretime_burn_pot), 500);

			// They're burnt at the end of the sale.
			advance_to(sale_start + timeslice_period * config.region_length + 1);
			assert_eq!(Balances::balance(coretime_burn_pot), 0);
		});
}
