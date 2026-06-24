// Copyright 2025 Limen-Neural
//
// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Ghost wallet — multi-asset virtual portfolio with Kelly position sizing.

use crate::engine::{CELLULAR_ATP, ENERGY_COMMITMENT};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Current market prices for all supported assets (USD).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MarketPrices {
    pub prices: HashMap<String, f32>,
}

impl MarketPrices {
    pub fn get(&self, asset: &str) -> f32 {
        *self.prices.get(asset).unwrap_or(&0.0)
    }
}

/// Virtual ghost-trading wallet with biological ATP energy model.
pub struct GhostWallet {
    // ── ATP base (cellular ATP) ────────────────────────────────────────
    pub balance_atp: f32,

    // ── Token positions ───────────────────────────────────────────────────
    pub balances: HashMap<String, f32>,
    pub entry_prices: HashMap<String, f32>,

    // ── Performance tracking ──────────────────────────────────────────────
    pub cumulative_pnl: f32,
    pub trade_count: u64,

    // ── Kelly criterion state ─────────────────────────────────────────────
    pub win_count: u64,
    pub loss_count: u64,
    pub total_win: f32,
    pub total_loss: f32,
    /// Adaptive trade fraction, initialized to `ENERGY_COMMITMENT`.
    pub trade_fraction: f32,
    pub price_history: VecDeque<f32>,
}

impl GhostWallet {
    /// Create a new ghost wallet with diversified initial positions.
    pub fn new() -> Self {
        Self {
            balance_atp: CELLULAR_ATP,
            balances: HashMap::new(),
            entry_prices: HashMap::new(),
            cumulative_pnl: 0.0,
            trade_count: 0,
            win_count: 0,
            loss_count: 0,
            total_win: 0.0,
            total_loss: 0.0,
            trade_fraction: ENERGY_COMMITMENT,
            price_history: VecDeque::with_capacity(50),
        }
    }

    /// Total portfolio value in quote units at current prices.
    pub fn portfolio_value(&self, prices: &HashMap<String, f32>) -> f32 {
        let mut total = self.balance_atp;
        for (asset, qty) in &self.balances {
            total += qty * prices.get(asset).unwrap_or(&0.0);
        }
        total
    }

    /// Get token balance for a named asset.
    pub fn balance(&self, asset: &str) -> f32 {
        *self.balances.get(asset).unwrap_or(&0.0)
    }

    /// Get cost basis (entry price) for a named asset.
    pub fn entry_price(&self, asset: &str) -> f32 {
        *self.entry_prices.get(asset).unwrap_or(&0.0)
    }

    /// Update balance and cost basis after a buy.
    pub(crate) fn apply_buy(&mut self, asset: &str, qty: f32, net_spend: f32) {
        let current_balance = *self.balances.get(asset).unwrap_or(&0.0);
        let current_entry = *self.entry_prices.get(asset).unwrap_or(&0.0);

        let prev_cost = current_entry * current_balance;
        let new_balance = current_balance + qty;

        self.balances.insert(asset.to_string(), new_balance);
        if new_balance > 1e-9 {
            self.entry_prices
                .insert(asset.to_string(), (prev_cost + net_spend) / new_balance);
        }
    }

    /// Reduce balance after a sell.
    pub(crate) fn apply_sell(&mut self, asset: &str, qty: f32) {
        let current_balance = *self.balances.get(asset).unwrap_or(&0.0);
        let new_balance = current_balance - qty;
        if new_balance > 1e-9 {
            self.balances.insert(asset.to_string(), new_balance);
            return;
        }
        self.balances.remove(asset);
        self.entry_prices.remove(asset);
    }

    /// Record closed trade PnL and update Kelly fraction.
    pub fn record_pnl_and_update_kelly(&mut self, pnl: f32) {
        if pnl > 0.0 {
            self.win_count += 1;
            self.total_win += pnl;
        } else if pnl < 0.0 {
            self.loss_count += 1;
            self.total_loss += pnl.abs();
        }

        let closed = self.win_count + self.loss_count;
        if closed < 10 {
            return;
        } // need minimum sample

        let win_rate = self.win_count as f64 / closed as f64;
        let avg_win = if self.win_count > 0 {
            self.total_win as f64 / self.win_count as f64
        } else {
            0.0
        };
        let avg_loss = if self.loss_count > 0 {
            self.total_loss as f64 / self.loss_count as f64
        } else {
            0.0
        };

        if avg_win < 1e-6 || avg_loss < 1e-6 {
            return;
        }

        let b = avg_win / avg_loss;
        let q = 1.0 - win_rate;
        let full_kelly = (win_rate * b - q) / b;
        let half_kelly = (full_kelly * 0.5).clamp(0.02, 0.20) as f32;
        self.trade_fraction = half_kelly;
    }

    /// Win rate from closed trades (returns `None` if no trades).
    pub fn win_rate(&self) -> Option<f64> {
        let closed = self.win_count + self.loss_count;
        if closed == 0 {
            None
        } else {
            Some(self.win_count as f64 / closed as f64)
        }
    }
}

impl Default for GhostWallet {
    fn default() -> Self {
        Self::new()
    }
}
