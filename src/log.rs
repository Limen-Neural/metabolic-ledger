// Copyright 2025 Limen-Neural
//
// Licensed under either of Apache License, Version 2.0 or MIT license at your option.
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use serde::Serialize;
use std::io::Write as _;

/// One ghost trade record, appended as a JSONL line.
#[derive(Debug, Serialize)]
pub struct GhostTradeLog {
    pub timestamp: String,
    pub step: u64,
    pub action: String,
    pub asset: String,
    pub price_usd: f32,
    pub quantity: f32,
    pub trade_value_usdt: f32,
    pub realized_pnl_usdt: f32,
    pub balance_atp: f32,
    pub cumulative_pnl: f32,
    pub reason: String,
}

/// Append one `GhostTradeLog` record to the given JSONL file path.
pub fn append_ghost_log(record: &GhostTradeLog, path: &str) {
    let Ok(line) = serde_json::to_string(record) else {
        return;
    };
    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(f, "{}", line);
    }
}
