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

//! Autogenerated weights for council
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
// --pallet=council
// --output=council.rs
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
		(0 as Weight) // Standard Error: 39_000
			.saturating_add((3_815_000 as Weight).saturating_mul(m as Weight)) // Standard Error: 39_000
			.saturating_add((139_000 as Weight).saturating_mul(n as Weight)) // Standard Error: 16_000
			.saturating_add((5_687_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn execute(b: u32, m: u32) -> Weight {
		(16_710_000 as Weight) // Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight)) // Standard Error: 3_000
			.saturating_add((100_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn propose_execute(b: u32, m: u32) -> Weight {
		(20_381_000 as Weight) // Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight)) // Standard Error: 4_000
			.saturating_add((155_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
		(27_857_000 as Weight) // Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight)) // Standard Error: 16_000
			.saturating_add((166_000 as Weight).saturating_mul(m as Weight)) // Standard Error: 6_000
			.saturating_add((392_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn vote(m: u32) -> Weight {
		(27_036_000 as Weight) // Standard Error: 10_000
			.saturating_add((151_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn close_early_disapproved(m: u32, p: u32) -> Weight {
		(29_500_000 as Weight) // Standard Error: 19_000
			.saturating_add((207_000 as Weight).saturating_mul(m as Weight)) // Standard Error: 6_000
			.saturating_add((349_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
		(38_838_000 as Weight) // Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight)) // Standard Error: 24_000
			.saturating_add((303_000 as Weight).saturating_mul(m as Weight)) // Standard Error: 8_000
			.saturating_add((360_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn close_disapproved(m: u32, p: u32) -> Weight {
		(30_672_000 as Weight) // Standard Error: 18_000
			.saturating_add((326_000 as Weight).saturating_mul(m as Weight)) // Standard Error: 6_000
			.saturating_add((384_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn close_approved(b: u32, m: u32, p: u32) -> Weight {
		(41_533_000 as Weight) // Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(b as Weight)) // Standard Error: 20_000
			.saturating_add((305_000 as Weight).saturating_mul(m as Weight)) // Standard Error: 6_000
			.saturating_add((383_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn disapprove_proposal(p: u32) -> Weight {
		(18_970_000 as Weight) // Standard Error: 10_000
			.saturating_add((352_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
