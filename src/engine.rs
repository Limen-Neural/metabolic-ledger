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

//! Ghost trade execution engine.

use crate::log::{GhostTradeLog, append_ghost_log};
use crate::wallet::GhostWallet;

/// Initial biological energy currency (quote units).
pub const CELLULAR_ATP: f32 = 500.0;

/// Fraction of available energy committed per trade signal (8%).
pub const ENERGY_COMMITMENT: f32 = 0.08;

/// Metabolic cost per action — models spread + slippage (0.1%).
pub const METABOLIC_COST: f32 = 0.001;

/// Execute a ghost buy order.
///
/// Spends `wallet.trade_fraction * wallet.balance_atp` ATP after deducting
/// the metabolic cost, then updates the weighted-average cost basis.
pub fn execute_buy(
    wallet: &mut GhostWallet,
    asset: &str,
    price: f32,
    step: u64,
    reason: &str,
    log_path: Option<&str>,
) {
    let spend_usdt = wallet.balance_atp * wallet.trade_fraction;
    if spend_usdt < 0.01 {
        return;
    }

    let fee = spend_usdt * METABOLIC_COST;
    let net_spend = spend_usdt - fee;
    let qty = net_spend / price.max(1e-9);

    wallet.apply_buy(asset, qty, net_spend);
    wallet.balance_atp -= spend_usdt;
    wallet.trade_count += 1;

    let record = GhostTradeLog {
        timestamp: chrono::Utc::now().to_rfc3339(),
        step,
        action: "buy".to_string(),
        asset: asset.to_string(),
        price_usd: price,
        quantity: qty,
        trade_value_usdt: spend_usdt,
        realized_pnl_usdt: -fee,
        balance_atp: wallet.balance_atp,
        cumulative_pnl: wallet.cumulative_pnl,
        reason: reason.to_string(),
    };

    eprintln!(
        "[ghost BUY ] step={:>5}  asset={:<6} qty={:.4} @ ${:.4}  fee=${:.4}",
        step, asset, qty, price, fee
    );

    if let Some(path) = log_path {
        append_ghost_log(&record, path);
    }
}

/// Execute a ghost sell order.
///
/// Sells `wallet.trade_fraction * balance[asset]` units, deducting metabolic cost.
pub fn execute_sell(
    wallet: &mut GhostWallet,
    asset: &str,
    price: f32,
    step: u64,
    reason: &str,
    log_path: Option<&str>,
) {
    let qty = wallet.balance(asset) * wallet.trade_fraction;
    if qty < 1e-9 {
        return;
    }

    let entry_price = wallet.entry_price(asset);
    let proceeds = qty * price;
    let fee = proceeds * METABOLIC_COST;
    let net_proceeds = proceeds - fee;
    let pnl = (price - entry_price) * qty - fee;

    wallet.apply_sell(asset, qty);
    wallet.balance_atp += net_proceeds;
    wallet.cumulative_pnl += pnl;
    wallet.trade_count += 1;
    wallet.record_pnl_and_update_kelly(pnl);

    let record = GhostTradeLog {
        timestamp: chrono::Utc::now().to_rfc3339(),
        step,
        action: "sell".to_string(),
        asset: asset.to_string(),
        price_usd: price,
        quantity: qty,
        trade_value_usdt: proceeds,
        realized_pnl_usdt: pnl,
        balance_atp: wallet.balance_atp,
        cumulative_pnl: wallet.cumulative_pnl,
        reason: reason.to_string(),
    };

    eprintln!(
        "[ghost SELL] step={:>5}  asset={:<6} qty={:.4} @ ${:.4}  PnL={:+.4}",
        step, asset, qty, price, pnl
    );

    if let Some(path) = log_path {
        append_ghost_log(&record, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_buy_reduces_usdt() {
        let mut wallet = GhostWallet::new();
        let before = wallet.balance_atp;
        execute_buy(&mut wallet, "ASSET_A", 0.03, 1, "test", None);
        assert!(wallet.balance_atp < before);
    }

    #[test]
    fn test_sell_increases_usdt() {
        let mut wallet = GhostWallet::new();
        wallet.balances.insert("ASSET_A".to_string(), 1_000.0);
        wallet.entry_prices.insert("ASSET_A".to_string(), 0.02);
        let before = wallet.balance_atp;
        execute_sell(&mut wallet, "ASSET_A", 0.05, 1, "test", None);
        assert!(
            wallet.balance_atp > before,
            "sell at profit should increase ATP"
        );
    }

    #[test]
    fn test_trade_count_increments() {
        let mut wallet = GhostWallet::new();
        execute_buy(&mut wallet, "ASSET_A", 90.0, 1, "test", None);
        execute_sell(&mut wallet, "ASSET_A", 95.0, 2, "test", None);
        assert_eq!(wallet.trade_count, 2);
    }

    #[test]
    fn test_portfolio_value() {
        let wallet = GhostWallet::new();
        let prices = HashMap::from([
            ("ASSET_A".to_string(), 12.5),
            ("ASSET_B".to_string(), 86.0),
            ("ASSET_C".to_string(), 1_500.0),
        ]);
        let value = wallet.portfolio_value(&prices);
        assert_eq!(value, wallet.balance_atp);
    }
}
