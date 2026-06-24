// Copyright 2025 Limen-Neural
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//!
//! Bio-inspired ghost trading engine with cellular ATP energy metaphors.
//!
//! Ghost trading simulates a multi-asset portfolio without real funds,
//! using biological energy concepts to constrain position sizing:
//!
//! - **ATP** (`CELLULAR_ATP = 500 quote units`) — total available energy
//! - **Energy commitment** (8% per signal) — fraction risked per trade
//! - **Metabolic cost** (0.1% per action) — friction / spread simulation
//!
//! All trades are logged to JSONL for SNN training data and performance analysis.
//!
//! ## Provenance
//!
//! Extracted from Eagle-Lander, the author's own private neuromorphic GPU supervisor repository (closed-source).
//! The ghost trading engine ran in production for multi-asset portfolio optimization before being open-sourced
//! as a standalone library.
//!
//! ## References
//!
//! - Kelly, J.L. (1956). A New Interpretation of Information Rate.
//!   *Bell System Technical Journal*, 35(4), 917–926.
//!   <https://doi.org/10.1002/j.1538-7305.1956.tb03809.x>
//!
//! - Thorp, E.O. (1969). Optimal Gambling Systems for Favorable Games.
//!   *Review of the International Statistical Institute*, 37(3), 273–293.
//!   Half-Kelly variance reduction rationale.
//!
//! - Alberts, B. et al. (2002). *Molecular Biology of the Cell* (4th ed.).
//!   ATP as cellular energy currency — conceptual basis for CELLULAR_ATP metaphor.

pub mod engine;
pub mod log;
pub mod wallet;

pub use engine::{CELLULAR_ATP, ENERGY_COMMITMENT, METABOLIC_COST, execute_buy, execute_sell};
pub use log::GhostTradeLog;
pub use wallet::{GhostWallet, MarketPrices};
