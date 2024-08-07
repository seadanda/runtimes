// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use asset_hub_kusama_emulated_chain;
pub use bridge_hub_kusama_emulated_chain;
pub use coretime_kusama_emulated_chain;
pub use kusama_emulated_chain;
pub use penpal_emulated_chain;
pub use people_kusama_emulated_chain;

use asset_hub_kusama_emulated_chain::AssetHubKusama;
use bridge_hub_kusama_emulated_chain::BridgeHubKusama;
use coretime_kusama_emulated_chain::CoretimeKusama;
use kusama_emulated_chain::Kusama;
use penpal_emulated_chain::{PenpalA, PenpalB};
use people_kusama_emulated_chain::PeopleKusama;

// Cumulus
use emulated_integration_tests_common::{
	accounts::{ALICE, BOB},
	xcm_emulator::{decl_test_networks, decl_test_sender_receiver_accounts_parameter_types},
};

decl_test_networks! {
	pub struct KusamaMockNet {
		relay_chain = Kusama,
		parachains = vec![
			AssetHubKusama,
			BridgeHubKusama,
			PenpalA,
			PenpalB,
			PeopleKusama,
			CoretimeKusama,
		],
		bridge = ()
	},
}

decl_test_sender_receiver_accounts_parameter_types! {
	KusamaRelay { sender: ALICE, receiver: BOB },
	AssetHubKusamaPara { sender: ALICE, receiver: BOB },
	BridgeHubKusamaPara { sender: ALICE, receiver: BOB },
	PenpalAPara { sender: ALICE, receiver: BOB },
	PenpalBPara { sender: ALICE, receiver: BOB },
	PeopleKusamaPara { sender: ALICE, receiver: BOB },
	CoretimeKusamaPara { sender: ALICE, receiver: BOB }
}
