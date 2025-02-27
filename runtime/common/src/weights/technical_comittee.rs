// This file is part of Hydra-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for tech
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-02, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// --chain=local
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=tech
// --output=technical_comittee.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_collective::weights::WeightInfo;

pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn set_members(m: u32, n: u32, p: u32) -> Weight {
		Weight::from_ref_time(0 as u64) // Standard Error: 27_000
			.saturating_add(Weight::from_ref_time(2_712_000 as u64).saturating_mul(m as u64)) // Standard Error: 27_000
			.saturating_add(Weight::from_ref_time(34_000 as u64).saturating_mul(n as u64)) // Standard Error: 13_000
			.saturating_add(Weight::from_ref_time(5_227_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
	fn execute(b: u32, m: u32) -> Weight {
		Weight::from_ref_time(16_914_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64)) // Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(104_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
	}
	fn propose_execute(b: u32, m: u32) -> Weight {
		Weight::from_ref_time(20_562_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64)) // Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(169_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
		Weight::from_ref_time(29_954_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(b as u64)) // Standard Error: 19_000
			.saturating_add(Weight::from_ref_time(89_000 as u64).saturating_mul(m as u64)) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(406_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn vote(m: u32) -> Weight {
		Weight::from_ref_time(24_765_000 as u64) // Standard Error: 12_000
			.saturating_add(Weight::from_ref_time(276_000 as u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn close_early_disapproved(m: u32, p: u32) -> Weight {
		Weight::from_ref_time(29_763_000 as u64) // Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(179_000 as u64).saturating_mul(m as u64)) // Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(417_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
		Weight::from_ref_time(40_677_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(b as u64)) // Standard Error: 31_000
			.saturating_add(Weight::from_ref_time(178_000 as u64).saturating_mul(m as u64)) // Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(390_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn close_disapproved(m: u32, p: u32) -> Weight {
		Weight::from_ref_time(33_154_000 as u64) // Standard Error: 23_000
			.saturating_add(Weight::from_ref_time(147_000 as u64).saturating_mul(m as u64)) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(424_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn close_approved(b: u32, m: u32, p: u32) -> Weight {
		Weight::from_ref_time(44_125_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(1_000 as u64).saturating_mul(b as u64)) // Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(126_000 as u64).saturating_mul(m as u64)) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(416_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn disapprove_proposal(p: u32) -> Weight {
		Weight::from_ref_time(19_027_000 as u64) // Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(374_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}
