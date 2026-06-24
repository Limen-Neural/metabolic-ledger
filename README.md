<p align="center">
  <img src="docs/logo.png" width="220" alt="Metabolic Ledger">
</p>

<h1 align="center">metabolic-ledger</h1>
<p align="center">Bio-inspired ghost trading engine with ATP cellular energy metaphors</p>

<p align="center">
  <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue" alt="MIT/Apache-2.0">
</p>

---

Simulates a multi-asset portfolio without real funds, using biological energy
concepts to constrain position sizing. Each trade signal consumes a fraction of
available ATP, with a metabolic cost modelling spread/slippage. All decisions are
logged to JSONL for SNN training data and post-hoc analysis.

## Features

- `GhostWallet` — dynamic multi-asset portfolio with weighted-average cost basis per asset
- `execute_buy` / `execute_sell` — ATP-gated order execution with metabolic cost
- `CELLULAR_ATP = 500.0` — initial energy budget (quote-unit equivalent)
- `ENERGY_COMMITMENT = 0.08` — 8% of available energy per signal
- `METABOLIC_COST = 0.001` — 0.1% friction per action
- `GhostTradeLog` — JSONL audit trail with timestamp, asset, side, price, units, reason
- Kelly fraction auto-update after 10+ trades based on realized win rate

## Review note
This branch contains the Wallet/MarketPrices refactor to dynamic asset maps.

## Usage (Git Dependency)

```toml
metabolic-ledger = { git = "https://github.com/Limen-Neural/metabolic-ledger" }
```

Recommended for reproducibility:

```toml
# Pin to a release tag
metabolic-ledger = { git = "https://github.com/Limen-Neural/metabolic-ledger", tag = "v0.1.0" }

# Or pin to an exact commit
metabolic-ledger = { git = "https://github.com/Limen-Neural/metabolic-ledger", rev = "<commit-sha>" }
```

## Quick Start

```rust
use metabolic_ledger::{GhostWallet, execute_buy, execute_sell};
use std::collections::HashMap;

let mut wallet = GhostWallet::new();
let prices: HashMap<String, f32> = HashMap::from([
    ("ASSET_A".to_string(), 12.5),
    ("ASSET_B".to_string(), 86.0),
]);

// SNN fires a BUY signal for ASSET_A
execute_buy(&mut wallet, "ASSET_A", prices["ASSET_A"], 1, "bull signal: confidence=0.92", None);

println!("Portfolio: {:.2}", wallet.portfolio_value(&prices));
println!("ASSET_A units: {:.2}", wallet.balance("ASSET_A"));
```

## With JSONL Audit Log

```rust
execute_buy(&mut wallet, "ASSET_A", 65.0, 1, "snn_fire", Some("trades.jsonl"));
// Appends: {"ts":"2024-...","asset":"ASSET_A","side":"BUY","price":65.0,...}
```

## Energy Model

```
available_energy = wallet.balance_atp
committed        = available_energy × ENERGY_COMMITMENT   (8%)
units            = committed / price
cost             = committed × METABOLIC_COST             (0.1% friction)
wallet.balance_atp     -= committed + cost
```

Inspired by ATP as cellular energy currency (Alberts et al. 2002) and half-Kelly
position sizing (Kelly 1956; Thorp 1969).

## Provenance

Extracted from Eagle-Lander, the author's own private neuromorphic GPU supervisor
repository (closed-source). Ghost-traded generalized multi-asset portfolios in
production alongside the live SNN supervisor.

## License

MIT OR Apache-2.0
